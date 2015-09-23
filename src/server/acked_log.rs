pub use server::{TXID, Term, PeerID};
use std::fmt;

use std::collections::BTreeMap;

pub trait AckedLog<T> {
    fn append(&mut self, term: Term, txid: TXID, entry: T);
    fn get(&self, txid: TXID) -> Option<T>;
    fn ack_up_to(&mut self, txid: TXID, peer: PeerID) -> Vec<(Term, TXID)>;
    fn commit_up_to(&mut self, txid: TXID) -> Vec<(Term, TXID)>;
    fn last_learned_term(&self) -> Term;
    fn last_learned_txid(&self) -> TXID;
    fn last_accepted_term(&self) -> Term;
    fn last_accepted_txid(&self) -> TXID;
}

// This should be used for testing and debugging only.
pub trait ViewableLog {
    fn acked(&self) -> Vec<(Term,TXID)>;
    fn learned(&self) -> Vec<(Term,TXID)>;
}

impl<T> fmt::Debug for AckedLog<T> + Send {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(lt: {} lx: {} at: {} ax: {})",
               self.last_learned_term(),
               self.last_learned_txid(),
               self.last_accepted_term(),
               self.last_accepted_txid())
    }
}

#[derive(Debug)]
pub struct LogEntry<T> {
    txid: TXID,
    term: Term,
    last_txid: TXID,
    last_term: Term,
    entry: T,
}

#[derive(Debug)]
pub struct Acked<T> {
    acks: Vec<PeerID>,
    inner: T,
}

// Leaders and Followers have an AckedLog for handling replication.
// Leaders have quorums of cluster_sz / 2 + 1, and Followers have
// a quorum of 1 (need a single subsequent ack from leader)
#[derive(Debug)]
pub struct InMemoryLog<T> {
    pub pending: BTreeMap<TXID, Acked<LogEntry<T>>>,
    pub committed: BTreeMap<TXID, LogEntry<T>>,
    pub quorum: usize,
    pub last_learned_txid: TXID,
    pub last_learned_term: Term,
    pub last_accepted_txid: TXID,
    pub last_accepted_term: Term,
}

unsafe impl<T> Sync for InMemoryLog<T>{}

impl<T: Clone> AckedLog<T> for InMemoryLog<T> {
    fn append(&mut self, term: Term, txid: TXID, entry: T) {
        self.pending.insert(txid, Acked{
            acks: vec![],
            inner: LogEntry {
                txid: txid,
                term: term,
                last_txid: self.last_accepted_txid,
                last_term: self.last_accepted_term,
                entry: entry,
            },
        });
        self.last_accepted_txid = txid;
        self.last_accepted_term = term;
    }

    fn get(&self, txid: TXID) -> Option<T> {
        self.pending.get(&txid)
            .map(|al| al.inner.entry.clone())
            .or(self.committed.get(&txid).map(|l| l.entry.clone()))
    }

    // Used by leaders to know when they've gotten enough acks.
    // returns a set of txid's that have reached quorum
    fn ack_up_to(&mut self, txid: TXID, peer: PeerID) -> Vec<(Term, TXID)> {
        // append ack
        for (txid, ent) in self.pending.iter_mut() {
            if ent.inner.txid <= *txid {
                if !ent.acks.contains(&peer) {
                    ent.acks.push(peer)
                }
                break
            }
        }
        let mut reached_quorum = vec![];
        loop {
            if self.pending.len() == 0 {
                break;
            }
            let txid = self.pending.keys().cloned().next().unwrap();
            if self.pending.get(&txid).unwrap().acks.len() < self.quorum {
                break;
            }
            // TODO(tyler) work out persistence story so we don't lose
            // logs during server crash between remove and push.
            let ent = self.pending.remove(&txid).unwrap();
            self.last_learned_term = ent.inner.term;
            self.last_learned_txid = ent.inner.txid;
            reached_quorum.push((ent.inner.term, ent.inner.txid));
            self.committed.insert(txid, ent.inner);
        }
        reached_quorum
    }

    // Used by followers to commit where the leader told them they should
    // be learning up to.
    // returns the set of txids that have reached quorum
    fn commit_up_to(&mut self, txid: TXID) -> Vec<(Term, TXID)> {
        let mut reached_quorum = vec![];
        loop {
            if self.pending.len() == 0 {
                break;
            }
            let next_txid = self.pending.keys().cloned().next().unwrap();
            if next_txid > txid {
                break;
            }
            let ent = self.pending.remove(&next_txid).unwrap();

            // TODO(tyler) work out persistence story so we don't lose
            // logs during server crash between remove and push.
            self.last_learned_term = ent.inner.term;
            self.last_learned_txid = ent.inner.txid;
            reached_quorum.push((ent.inner.term, ent.inner.txid));
            self.committed.insert(txid, ent.inner);
        }
        reached_quorum
    }

    fn last_learned_term(&self) -> Term {
        self.last_learned_term
    }

    fn last_learned_txid(&self) -> TXID {
        self.last_learned_txid
    }

    fn last_accepted_term(&self) -> Term {
        self.last_accepted_term
    }

    fn last_accepted_txid(&self) -> TXID {
        self.last_accepted_txid
    }
}

impl<T: Clone> ViewableLog for InMemoryLog<Acked<LogEntry<T>>> {
    fn acked(&self) -> Vec<(Term,TXID)> {
        let mut ret = vec![];
        for (txid, acked) in self.pending.iter() {
            ret.push((acked.inner.term, *txid));
        }
        ret
    }

    fn learned(&self) -> Vec<(Term,TXID)> {
        let mut ret = vec![];
        for (txid, learned) in self.committed.iter() {
            ret.push((learned.term, *txid));
        }
        ret
    }
}

use std::cmp::Ordering;

pub struct RangeBounds {
    lower: Vec<u8>,
    upper: Vec<u8>,
}

impl RangeBounds {
    pub fn new(lower: Vec<u8>, upper: Vec<u8>) -> Result<RangeBounds, String> {
        if lower >= upper {
            Err("lower is >= upper, which is incorrect!".to_string())
        } else {
            Ok(RangeBounds {
                lower: lower,
                upper: upper,
            })
        }
    }
}

impl Ord for RangeBounds {
    fn cmp(&self, other: &Self) -> Ordering {
        assert!(self.upper > self.lower);
        assert!(other.upper > other.lower);
        if self.upper <= other.lower {
            Ordering::Less
        } else if self.lower >= other.upper {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for RangeBounds {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RangeBounds {
    fn eq(&self, other: &Self) -> bool {
        (&self.lower, &self.upper) == (&other.lower, &other.upper)
    }
}

impl Eq for RangeBounds { }

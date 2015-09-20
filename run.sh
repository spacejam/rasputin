#!/bin/sh
peers="127.0.0.1:7770,127.0.0.1:7771,127.0.0.1:7772,127.0.0.1:7773,127.0.0.1:7774"
for i in {0..4}; do
  mkdir -p _rasputin_test/$i/data
  target/debug/rasputind \
    --peer-port=777$i \
    --cli-port=888$i \
    --seed-peers=$peers \
    --storage-dir=_rasputin_test/$i/data \
    --logfile=_rasputin_test/$i.log &
done

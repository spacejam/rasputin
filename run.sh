#!/bin/sh
mkdir _test
peers="127.0.0.1:7770,127.0.0.1:7771,127.0.0.1:7772,127.0.0.1:7773,127.0.0.1:7774"
for i in {0..4}; do mkdir _test/$i; target/debug/rasputind --peer-port=777$i --cli-port=888$i --peers=$peers --storage-dir=_test/$i/data --logfile=_test/$i.log & done

#!/bin/sh
mkdir _test
peers="localhost:7770,localhost:7771,localhost:7772,localhost:7773,localhost:7774"
for i in {0..4}; do mkdir _test/$i; target/debug/rasputind --peer-port=777$i --cli-port=888$i --peers=$peers --storage-dir=_test/$i/data & done

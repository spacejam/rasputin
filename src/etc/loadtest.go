package main

import (
	"bufio"
	"fmt"
	"net"
	"runtime"

	"github.com/spacejam/loghisto"
)

func benchmark() {
	conn, err := net.Dial("tcp", "localhost:8880")
	if err != nil {
		fmt.Errorf("could not connect: %v", err)
		return
	}
	fmt.Fprintf(conn, "\x00\x00\x00\x03yo\n")
	_, err = bufio.NewReader(conn).ReadString('\n')
	if err != nil {
		fmt.Errorf("could not read response: %v", err)
		return
	}
}

func main() {
	numCPU := runtime.NumCPU()
	runtime.GOMAXPROCS(numCPU)

	desiredConcurrency := uint(10)
	loghisto.PrintBenchmark("benchmark1234", desiredConcurrency, benchmark)
}

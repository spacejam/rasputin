package main

import (
	"bufio"
	"fmt"
	"net"
	"runtime"

	"github.com/spacejam/loghisto"
)

func benchmark(conn net.Conn) {
	fmt.Fprintf(conn, "\x00\x00\x00\x03yo\n")
	r, err := bufio.NewReader(conn).ReadString('\n')
	if err != nil {
		fmt.Errorf("could not read response: %v", err)
		return
	}
	if r != "\x00\x00\x00\x03yo\n" {
		fmt.Println("bad response")
	}
}

func main() {
	numCPU := runtime.NumCPU()
	runtime.GOMAXPROCS(numCPU)

	fire := make(chan struct{})
	for i := 0; i < 50; i++ {
		go func() {
			conn, err := net.Dial("tcp", "localhost:8880")
			if err != nil {
				fmt.Errorf("could not connect: %v", err)
				return
			}
			for {
				<-fire
				benchmark(conn)
			}
		}()
	}

	desiredConcurrency := uint(10)
	loghisto.PrintBenchmark("benchmark1234", desiredConcurrency, func() { fire <- struct{}{} })
}

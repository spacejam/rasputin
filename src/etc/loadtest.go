package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
)

func main() {
	conn, err := net.Dial("tcp", "localhost:7771")
	if err != nil {
		fmt.Errorf("could not connect: %v", err)
		os.Exit(1)
	}
	fmt.Fprintf(conn, "\x00\x00\x00\x03yo\n")
	status, err := bufio.NewReader(conn).ReadString('\n')
	if err != nil {
		fmt.Errorf("could not read response: %v", err)
		os.Exit(1)
	}
	fmt.Printf("got response: %s\n", status)
}

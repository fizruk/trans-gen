package main

import (
	"bufio"
	. "trans_gen_test/codegame"
	"os"
	"fmt"
	"net"
	"strconv"
	. "trans_gen_test/stream"
)

func main() {
	host := os.Args[1]
	portInt, err := strconv.Atoi(os.Args[2])
	if err != nil {
		panic(err)
	}
	port := uint16(portInt)
	stdout, err := strconv.ParseBool(os.Args[3])
	if err != nil {
		panic(err)
	}

	conn, err := net.Dial("tcp", host+":"+strconv.Itoa(int(port)))
	if err != nil {
		panic(err)
	}
	reader := bufio.NewReader(conn)
	writer := bufio.NewWriter(conn)

	for ReadBool(reader) {
		input := ReadMessageGameModel(reader)

		if stdout {
			fmt.Println(input)
		}

		input.Write(writer)
		err = writer.Flush()
		if err != nil {
			panic(err)
		}
	}

	err = conn.Close()
	if err != nil {
		panic(err)
	}
}
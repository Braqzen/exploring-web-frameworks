package main

import (
	"log"
	"os"
)

func main() {
	socket, ok := os.LookupEnv("SOCKET")
	if !ok || socket == "" {
		log.Fatal("Socket error")
	}

	server, err := NewServer(socket)
	if err != nil {
		log.Fatal("Bad socket")
	}

	err = server.Run()
	if err != nil {
		log.Fatal("Server crash")
	}
}

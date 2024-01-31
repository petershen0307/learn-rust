package main

import (
	"bufio"
	"context"
	"fmt"
	"log"
	"net"
	"os"
	"os/signal"
	"sync"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, os.Interrupt)
	go func() {
		<-sigChan
		log.Println("Graceful shutdown...")
		cancel()
	}()
	listenAndResponse(ctx)
	log.Println("Server exited")
}

func listenAndResponse(ctx context.Context) {
	ln, err := net.Listen("tcp", ":8080")
	if err != nil {
		log.Fatalln(err)
	}
	go func() {
		<-ctx.Done()
		ln.Close()
	}()
	waitGroup := sync.WaitGroup{}
	for {
		conn, err := ln.Accept()
		if err != nil {
			break
		}
		go handleClient(ctx, conn, &waitGroup)
		waitGroup.Add(1)
	}
	log.Println("Waiting for all clients to be done...")
	waitGroup.Wait()
}

func handleClient(ctx context.Context, conn net.Conn, wg *sync.WaitGroup) {
	defer wg.Done()
	go func() {
		<-ctx.Done()
		conn.Close()
	}()
	scanner := bufio.NewScanner(conn)
	for scanner.Scan() {
		token := scanner.Text()
		log.Println("from:", conn.RemoteAddr(), "received msg:", token)
		fmt.Fprintln(conn, "echo: ", token)
	}
	log.Println("from:", conn.RemoteAddr(), "Connection closed")
}

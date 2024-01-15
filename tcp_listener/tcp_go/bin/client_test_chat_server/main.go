package main

import (
	"fmt"
	"log"
	"net"
	"os"
	"strconv"
	"sync"
	"time"
)

func parseCommand() *command {
	port, _ := strconv.Atoi(os.Args[2])
	concurrent, _ := strconv.Atoi(os.Args[3])
	return &command{
		host:       os.Args[1],
		port:       port,
		concurrent: concurrent,
		metrics:    make([]metric, 0),
	}
}

type metric struct {
	id        int
	startTime time.Time
	endTime   time.Time
}

type command struct {
	host       string
	port       int
	concurrent int
	metrics    []metric
}

func (c *command) run() {
	wg := sync.WaitGroup{}
	for i := 0; i < c.concurrent; i++ {
		go c.oneTest(i, &wg)
		wg.Add(1)
	}
	wg.Wait()
	c.report()
}

func (c *command) oneTest(concurrentNumber int, wg *sync.WaitGroup) {
	metric := metric{
		id:        concurrentNumber,
		startTime: time.Now(),
	}
	connection, _ := net.Dial("tcp", fmt.Sprintf("%s:%d", c.host, c.port))
	defer connection.Close()
	for i := 0; i < 100; i++ {
		_, _ = connection.Write([]byte(fmt.Sprintf("broadcast:hello %d\n", i)))
	}
	metric.endTime = time.Now()
	c.metrics = append(c.metrics, metric)
	wg.Done()
}

func (c *command) report() {
	diff := time.Duration(0)
	for _, metric := range c.metrics {
		diff += metric.endTime.Sub(metric.startTime)
	}
	fmt.Println("Total time(ms):", diff.Milliseconds())
	fmt.Println("Average time(ms):", (diff / time.Duration(len(c.metrics))).Milliseconds())
}

func main() {
	if len(os.Args) != 4 {
		log.Fatalln("Usage: client_test_connection <host> <port> <concurrent number>")
	}
	command := parseCommand()
	command.run()
}

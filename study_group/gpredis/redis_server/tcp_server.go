package redisserver

import (
	"bytes"
	"context"
	"io"
	"log"
	"net"

	datawatcher "gpredis/data_watcher"
	redisprotocolanalyzer "gpredis/redis_protocol_analyzer"

	"github.com/tidwall/resp"
)

type server struct {
	dataChan chan datawatcher.ChannelMessage
}

func NewChannelVersion(dataChan chan datawatcher.ChannelMessage) *server {
	return &server{dataChan: dataChan}
}

func (s *server) Serve(ctx context.Context) {
	listener, err := net.ListenTCP("tcp", &net.TCPAddr{
		IP:   net.IPv4(0, 0, 0, 0),
		Port: 6379,
	})
	if err != nil {
		log.Fatal(err)
		return
	}
	go func() {
		<-ctx.Done()
		listener.Close()
	}()
	for {
		if conn, err := listener.AcceptTCP(); err == nil {
			go s.connectionHandle(ctx, conn)
		} else {
			break
		}
	}
}

func (s *server) connectionHandle(ctx context.Context, conn net.Conn) {
	go func() {
		<-ctx.Done()
		conn.Close()
	}()
	for {
		data, err := readAll(conn)
		if err != nil {
			conn.Close()
			return
		}
		command, err := redisprotocolanalyzer.Parse(data)
		if err != nil {
			d, _ := resp.ErrorValue(err).MarshalRESP()
			if _, err := io.Copy(conn, bytes.NewReader(d)); err != nil {
				log.Println(err)
			}
			continue
		}
		callback := make(chan resp.Value)
		s.dataChan <- datawatcher.ChannelMessage{
			Command:  command,
			Callback: callback,
		}
		v := <-callback
		r, _ := v.MarshalRESP()
		if _, err := io.Copy(conn, bytes.NewReader(r)); err != nil {
			log.Println(err)
		}
	}
}

func readAll(r io.Reader) ([]byte, error) {
	output := []byte{}
	buf := make([]byte, 512)
	for {
		n, err := r.Read(buf)
		output = append(output, buf[:n]...)
		if err != nil || n == 0 {
			return nil, err
		}
		if n < len(buf) {
			return output, nil
		}
	}
}

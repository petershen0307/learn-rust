package main

import (
	"context"
	"os"
	"os/signal"

	datawatcher "gpredis/data_watcher"
	redisserver "gpredis/redis_server"
)

func main() {
	ctx, _ := signal.NotifyContext(context.Background(), os.Interrupt)
	dataChan := make(chan datawatcher.ChannelMessage, 10)
	go func() {
		<-ctx.Done()
		close(dataChan)
	}()
	go datawatcher.Run(dataChan)
	redisserver.NewChannelVersion(dataChan).Serve(ctx)
}

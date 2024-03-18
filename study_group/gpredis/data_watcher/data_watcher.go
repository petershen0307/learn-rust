package datawatcher

import (
	redisprotocolanalyzer "gpredis/redis_protocol_analyzer"

	"github.com/tidwall/resp"
)

type ChannelMessage struct {
	Command  redisprotocolanalyzer.Command
	Callback chan resp.Value
}

func Run(msgChan chan ChannelMessage) {
	storage := make(redisprotocolanalyzer.ChannelDataStorage)
	for msg := range msgChan {
		r := msg.Command.Exec(&storage)
		msg.Callback <- r
	}
}

package redisprotocolanalyzer

import (
	"bytes"
	"fmt"
	"strings"

	"github.com/tidwall/resp"
)

type ChannelDataStorage map[string]string

type Command interface {
	Exec(*ChannelDataStorage) resp.Value
}

func Parse(data []byte) (Command, error) {
	r := resp.NewReader(bytes.NewReader(data))
	v, _, err := r.ReadValue()
	if err != nil {
		return nil, err
	}
	if v.Type() != resp.Array {
		return nil, fmt.Errorf("invalid syntax")
	}
	cmd := []string{}
	for _, i := range v.Array() {
		switch i.Type() {
		case resp.SimpleString, resp.BulkString:
			cmd = append(cmd, i.String())
		}
	}
	switch strings.ToLower(cmd[0]) {
	case "command":
		return Cmd{}, nil
	case "set":
		return Set{
			key:   cmd[1],
			value: cmd[2],
		}, nil
	case "get":
		return Get{
			key: cmd[1],
		}, nil
	}
	return nil, fmt.Errorf("invalid syntax")
}

type Cmd struct{}

func (c Cmd) Exec(d *ChannelDataStorage) resp.Value {
	return resp.SimpleStringValue("ok")
}

type Set struct {
	key   string
	value string
}

func (s Set) Exec(d *ChannelDataStorage) resp.Value {
	(*d)[s.key] = s.value
	return resp.SimpleStringValue("ok")
}

type Get struct {
	key string
}

func (g Get) Exec(d *ChannelDataStorage) resp.Value {
	if v, ok := (*d)[g.key]; ok {
		return resp.StringValue(v)
	}
	return resp.NullValue()
}

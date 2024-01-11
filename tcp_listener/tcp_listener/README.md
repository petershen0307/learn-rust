# tcp listener

1. write tcp listener only serve one connection at once
2. try to use thread to handle multiple connection
3. try to use single thread to handle multiple connection
4. tcp listener will wait 5 second and expect tcp client will send a message, if client didn't send the message, server send a message to client

additional, how to do the graceful shutdown in every challenge?

```bash
# tcp server and client tool
# client
nc 127.0.0.1 8080
# server
nc -l 8081
```

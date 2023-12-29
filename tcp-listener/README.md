# tcp listener

1. write a blocking tcp listener
2. try to use thread to achieve non-blocking handler
3. try not to use thread to achieve non-blocking handler
4. tcp listener will wait 5 second and expect tcp client will send a message, if client didn't send the message, server send a message to client

```bash
# tcp server and client tool
# client
nc 127.0.0.1 8080
# server
nc -l 8081
```

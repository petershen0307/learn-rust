# mini session manager

The project want to reproduce like aws session manager system.
[AWS Supports You | Diving Deep into AWS Systems Manager](https://www.youtube.com/watch?v=xHNLNTa2xGU)
[AWS Systems Manager Session Manager](https://docs.aws.amazon.com/systems-manager/latest/userguide/session-manager.html)

The architecture will be one server and two clients.
The server will response for receive the command from client and pass to another client.
One client will be sending command another client will be receive command and print to console.
The network protocol want to use http2 persistent connection.

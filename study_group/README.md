# This repo is used to collect codes from study group


## Week 1 - Sync/Async TCPServer
## Week 2 - Two way communication TCPServer
## Week 3 - Chat Server
* direct chat
* broadcast chat
## Week 4 - Socks4 Server
* https://www.openssh.com/txt/socks4.protocol
* Implement "Connect"
## Week 5 - Socks4 Server
* Implement "Bind"
## Week 6 - Web service by framework
* Implement web service
    * **POST** api/v1/filehasher
        * body is the path (ex. `/Users/kiwi/target`)
    * Calucate **SHA512** of given directory, output should be sorted by filename
        * use the path got from request `/Users/kiwi/target`
        * if there are 3 files in the directory with filename `2.pdf`, `1.txt`, `3.doc`
        * the response should be like the following
 ```
/Users/kiwi/target/1.txt <SHA512 of 1.txt>
/Users/kiwi/target/2.pdf <SHA512 of 2.pdf>
/Users/kiwi/target/3.doc <SHA512 of 3.doc>
```
## Week 7 - Revisit performance week 6
## Week 8 - Revisit performance week 7
## Week 9 - Little Redis
* Build a single/standalone server for serving redis client
   * Support only SET/GET/DEL for 1st week 

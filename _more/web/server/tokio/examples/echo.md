# echo

## echo server

```
PS D:\ccc\code\00\rust\web\server\tokio\examples> cargo run --example echo
   Compiling examples v0.0.0 (D:\ccc\code\00\rust\web\server\tokio\examples)
    Finished dev [unoptimized + debuginfo] target(s) in 29.32s
     Running `target\debug\examples\echo.exe`
Listening on: 127.0.0.1:8080
```

## connect

```
PS D:\ccc\code\00\rust\web\server\tokio\examples> cargo run --example connect 127.0.0.1:8080
   Compiling examples v0.0.0 (D:\ccc\code\00\rust\web\server\tokio\examples)
    Finished dev [unoptimized + debuginfo] target(s) in 11.08s                                                
     Running `target\debug\examples\connect.exe 127.0.0.1:8080`
hello
hello
hi
hi
I am ccc
I am ccc
你好
你好

```

## client 1

```
Please enter your username:
                           ccckmit

snoopy has joined the chat
                          snoopy: hi
                                    snoopy: , how are you
                                                         snoopy: , i am snoopy,

```

## client 2

```
Please enter your username:
                           snoopy
hi
, how are you
, i am snoopy,
,
```


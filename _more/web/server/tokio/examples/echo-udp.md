# echo-udp

## server

```
PS D:\ccc\code\00\rust\web\server\tokio\examples> cargo run --example echo-udp
   Compiling examples v0.0.0 (D:\ccc\code\00\rust\web\server\tokio\examples)
    Finished dev [unoptimized + debuginfo] target(s) in 6.05s
     Running `target\debug\examples\echo-udp.exe`
Listening on: 127.0.0.1:8080
```

## client

```
PS D:\ccc\code\00\rust\web\server\tokio\examples> cargo run --example connect -- --udp 127.0.0.1:8080
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
     Running `target\debug\examples\connect.exe --udp 127.0.0.1:8080`
hello
hello

hi
hi

中文
中文

error: process didn't exit successfully: `target\debug\examples\connect.exe --udp 127.0.0.1:8080` (exit code: 
0xc000013a, STATUS_CONTROL_C_EXIT)
```
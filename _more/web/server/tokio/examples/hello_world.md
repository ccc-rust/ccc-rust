# hello_world

先執行 nc

```
PS D:\ccc\code\00\rust\web\server\tokio\examples> nc -l -p 6142
```

然後再開另一視窗執行 hello_world

```
PS D:\ccc\code\00\rust\web\server\tokio\examples> cargo run --example hello_world
    Finished dev [unoptimized + debuginfo] target(s) in 10.19s
     Running `target\debug\examples\hello_world.exe`
created stream
wrote to stream; success=true
```

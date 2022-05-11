## 費氏數列 -- 用查表加速

傳統用遞迴方式的費氏數列算法，會耗費很久的時間：

執行結果:

```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/alg/01-tableLookup/fibonacci (master)
$ rustc fibonacci.rs

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/alg/01-tableLookup/fibonacci (master)
$ ./fibonacci
start_time:SystemTime { intervals: 132392510304662048 }
fibonacci(38)=39088169
end_time:SystemTime { intervals: 132392510321390946 }
duration:1.6728898s
```

加入查表，讓已經算過的就不需要算第二次，第二次之後改用查的！

執行結果

```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/alg/01-tableLookup/fibonacci (master)
$ rustc fibonacci_lookup.rs

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/alg/01-tableLookup/fibonacci (master)
$ ./fibonacci_lookup
start_time:SystemTime { intervals: 132392510982519643 }
fibonacci(38)=39088169
end_time:SystemTime { intervals: 132392510982570619 }
duration:5.0976ms
```

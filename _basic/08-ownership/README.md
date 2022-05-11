# 所有權、引用與借用

## 所有權

每個物件都有唯一的所有者。

```rs
y=x 
```

這樣的語句會造成所有權轉移！

## 引用

但是改成引用就不會轉移。

```rs
let y = &x;
```

* https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html

## 借用 borrowing

```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/_basic/08-ownership (master)
$ rustc borrow1.rs

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/_basic/08-ownership (master)
$ ./borrow1
e2c.get(dog)=Some("狗")
e2c.get(xxx)=None
```

# cargo publish

* [Rust 學習之路─第七章：模組](https://magiclen.org/rust-module/)
* [将 crate 发布到 Crates.io](https://kaisery.github.io/trpl-zh-cn/ch14-02-publishing-to-crates-io.html)


```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/se/publish/smap (master)
$ cargo login xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
       Login token for `crates.io` saved

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/se/publish/smap (master)
$ cargo publish
    Updating crates.io index
   Packaging smap v0.1.0 (D:\ccc\ccc109a\se\rust\se\publish\smap)
   Verifying smap v0.1.0 (D:\ccc\ccc109a\se\rust\se\publish\smap)
   Compiling smap v0.1.0 (D:\ccc\ccc109a\se\rust\se\publish\smap\target\package\smap-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 4.47s
   Uploading smap v0.1.0 (D:\ccc\ccc109a\se\rust\se\publish\smap)
```

## 

```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/se/publish/smap (master)
$ git add -A
warning: LF will be replaced by CRLF in Cargo.lock.
The file will have its original line endings in your working directory.
warning: LF will be replaced by CRLF in Cargo.toml.
The file will have its original line endings in your working directory.

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/se/publish/smap (master)
$ git commit -m "v0.1.1"
[master c64b101] v0.1.1
 2 files changed, 2 insertions(+), 2 deletions(-)

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/se/publish/smap (master)
$ git push origin master
fatal: HttpRequestException encountered.
   傳送要求時發生錯誤。
Username for 'https://github.com': ccckmit
Password for 'https://ccckmit@github.com':
Counting objects: 4, done.
Delta compression using up to 4 threads.
Compressing objects: 100% (4/4), done.
Writing objects: 100% (4/4), 460 bytes | 153.00 KiB/s, done.
Total 4 (delta 2), reused 0 (delta 0)
remote: Resolving deltas: 100% (2/2), completed with 2 local objects.
To https://github.com/ccckmit/smap.git
   a225ec6..c64b101  master -> master

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/se/publish/smap (master)
$ cargo publish
    Updating crates.io index
   Packaging smap v0.1.1 (D:\ccc\ccc109a\se\rust\se\publish\smap)
   Verifying smap v0.1.1 (D:\ccc\ccc109a\se\rust\se\publish\smap)
   Compiling smap v0.1.1 (D:\ccc\ccc109a\se\rust\se\publish\smap\target\package\smap-0.1.1)
    Finished dev [unoptimized + debuginfo] target(s) in 6.13s
   Uploading smap v0.1.1 (D:\ccc\ccc109a\se\rust\se\publish\smap)
```
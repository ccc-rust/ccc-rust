# Call Assembly

* https://stackoverflow.com/questions/47455163/how-to-call-assembly-functions-inside-rust

## 失敗

```
user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/_more/sp/call/callasm (master)
$ cargo run
    Updating crates.io index
  Downloaded cc v1.0.58
   Compiling cc v1.0.58
   Compiling callasm v0.1.0 (D:\ccc\ccc109a\se\rust\_more\sp\call\callasm)
error: failed to run custom build command for `callasm v0.1.0 (D:\ccc\ccc109a\se\rust\_more\sp\call\callasm)`

Caused by:
  process didn't exit successfully: `D:\ccc\ccc109a\se\rust\_more\sp\call\callasm\target\debug\build\callasm-b141357c92da7bd3\build-script-build` (exit code: 1)
--- stderr


error occurred: Command "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Community\\VC\\Tools\\MSVC\\14.25.28610\\bin\\HostX64\\x64\\lib.exe" "-out:D:\\ccc\\ccc109a\\se\\rust\\_more\\sp\\call\\callasm\\target\\debug\\build\\callasm-90afeffc1315d14c\\out\\libmy-asm-lib.a" "-nologo" "D:\\ccc\\ccc109a\\se\\rust\\_more\\sp\\call\\callasm\\target\\debug\\build\\callasm-90afeffc1315d14c\\out\\add.o" with args "lib.exe" did not execute successfully (status code exit code: 1181).
```
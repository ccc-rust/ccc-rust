# call c - 有 embed assembly

在 windows 中，失敗:


```

user@DESKTOP-96FRN6B MINGW64 /d/ccc/ccc109a/se/rust/_more/sp/call/callc3asm (master)
$ cargo run
   Compiling callc v0.1.0 (D:\ccc\ccc109a\se\rust\_more\sp\call\callc3asm)
error: failed to run custom build command for `callc v0.1.0 (D:\ccc\ccc109a\se\rust\_more\sp\call\callc3asm)`

Caused by:
  process didn't exit successfully: `d:\ccc\ccc109a\se\rust\_more\sp\call\callc3asm\target\debug\build\callc-d898687200cc21a6\build-script-build` (exit code: 1)
--- stderr


error occurred: Command "C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\Community\\VC\\Tools\\MSVC\\14.25.28610\\bin\\HostX64\\x64\\cl.exe" "-nologo" "-MD" "-Z7" "-Brepro" "-W4" "-Fod:\\ccc\\ccc109a\\se\\rust\\_more\\sp\\call\\callc3asm\\target\\debug\\build\\callc-24d22dcb2728b6c3\\out\\add.o" "-c" "add.c" with args "cl.exe" did not execute successfully (status code exit code: 2).


```

似乎是用 VC 去編譯，但我們的語法卻是 gnu 的，所以失敗！



# Expected output

```
PS C:\Rust\framehandler> cargo +1.59 run --release
   Compiling framehandler v0.1.0 (C:\Rust\framehandler)
error: linking with `link.exe` failed: exit code: 1120
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.32.31326\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "C:\\Rust\\framehandler\\target\\release\\deps\\framehandler.framehandler.896e69f6-cgu.0.rcgu.o" "/LIBPATH:C:\\Rust\\framehandler\\target\\release\\deps" "/LIBPATH:C:\\Rust\\framehandler\\libs" "/LIBPATH:C:\\Users\\chris\\.rustup\\toolchains\\1.59-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\chris\\.rustup\\toolchains\\1.59-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-18761c3bc8f2e6ea.rlib" "kernel32.lib" "ws2_32.lib" "bcrypt.lib" "advapi32.lib" "userenv.lib" "kernel32.lib" "msvcrt.lib" "/NXCOMPAT" "/LIBPATH:C:\\Users\\chris\\.rustup\\toolchains\\1.59-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/OUT:C:\\Rust\\framehandler\\target\\release\\deps\\framehandler.exe" "/OPT:REF,ICF" "/DEBUG" "/NATVIS:C:\\Users\\chris\\.rustup\\toolchains\\1.59-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:C:\\Users\\chris\\.rustup\\toolchains\\1.59-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:C:\\Users\\chris\\.rustup\\toolchains\\1.59-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:C:\\Users\\chris\\.rustup\\toolchains\\1.59-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libstd.natvis"
  = note: framehandler.framehandler.896e69f6-cgu.0.rcgu.o : error LNK2019: unresolved external symbol memcmp referenced in function _ZN4core3str7pattern11StrSearcher3new17h3ed25b3b8d8a4e05E
          framehandler.framehandler.896e69f6-cgu.0.rcgu.o : error LNK2019: unresolved external symbol _tls_index referenced in function main
          framehandler.framehandler.896e69f6-cgu.0.rcgu.o : error LNK2019: unresolved external symbol memset referenced in function _ZN64_$LT$rustc_demangle..v0..Ident$u20$as$u20$core..fmt..Display$GT$3fmt17hd238cf769d753164E
          framehandler.framehandler.896e69f6-cgu.0.rcgu.o : error LNK2019: unresolved external symbol _tls_used referenced in function _ZN3std3sys7windows16thread_local_key15on_tls_callback17h2d05f37fb90d46e8E
          framehandler.framehandler.896e69f6-cgu.0.rcgu.o : error LNK2019: unresolved external symbol memcpy referenced in function _ZN3std10sys_common4wtf87Wtf8Buf25push_code_point_unchecked17h161d12097b65ab70E
          framehandler.framehandler.896e69f6-cgu.0.rcgu.o : error LNK2019: unresolved external symbol __chkstk referenced in function _ZN3std3sys7windows5stdio27write_valid_utf8_to_console17h1edc69169f9be598E
          framehandler.framehandler.896e69f6-cgu.0.rcgu.o : error LNK2019: unresolved external symbol memmove referenced in function _ZN3std4sync4once4Once9call_once28_$u7b$$u7b$closure$u7d$$u7d$17hed4be38205c38f6bE
          LINK : error LNK2001: unresolved external symbol mainCRTStartup
          C:\Rust\framehandler\target\release\deps\framehandler.exe : fatal error LNK1120: 8 unresolved externals


error: could not compile `framehandler` due to previous error
```

# Actual output (1.60+)

Notice `unresolved external symbol __CxxFrameHandler3`.

```
PS C:\Rust\framehandler> cargo run --release      
   Compiling framehandler v0.1.0 (C:\Rust\framehandler)
error: linking with `link.exe` failed: exit code: 1120
  |
  = note: "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.32.31326\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "C:\\Users\\chris\\AppData\\Local\\Temp\\rustcPv0Bly\\symbols.o" "C:\\Rust\\framehandler\\target\\release\\deps\\framehandler.framehandler.07f64a4f-cgu.2.rcgu.o" "/LIBPATH:C:\\Rust\\framehandler\\target\\release\\deps" "/LIBPATH:C:\\Rust\\framehandler\\libs" "/LIBPATH:C:\\Users\\chris\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\chris\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-c045e84c0343a063.rlib" "advapi32.lib" "userenv.lib" "kernel32.lib" "ws2_32.lib" "bcrypt.lib" "msvcrt.lib" "/NXCOMPAT" "/LIBPATH:C:\\Users\\chris\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "/OUT:C:\\Rust\\framehandler\\target\\release\\deps\\framehandler.exe" "/OPT:REF,ICF" "/DEBUG" "/NATVIS:C:\\Users\\chris\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:C:\\Users\\chris\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:C:\\Users\\chris\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libcore.natvis" "/NATVIS:C:\\Users\\chris\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libstd.natvis"
  = note: framehandler.framehandler.07f64a4f-cgu.2.rcgu.o : error LNK2001: unresolved external symbol __CxxFrameHandler3
          framehandler.framehandler.07f64a4f-cgu.2.rcgu.o : error LNK2019: unresolved external symbol memcmp referenced in function _ZN4core3str7pattern11StrSearcher3new17h00a47f5a7c278547E
          framehandler.framehandler.07f64a4f-cgu.2.rcgu.o : error LNK2019: unresolved external symbol _tls_index referenced in function main
          framehandler.framehandler.07f64a4f-cgu.2.rcgu.o : error LNK2019: unresolved external symbol memset referenced in function _ZN64_$LT$rustc_demangle..v0..Ident$u20$as$u20$core..fmt..Display$GT$3fmt17h54ece988f5e81c7aE
          framehandler.framehandler.07f64a4f-cgu.2.rcgu.o : error LNK2019: unresolved external symbol _tls_used referenced in function _ZN3std3sys7windows16thread_local_key15on_tls_callback17hca5bb6a666fe4695E
          framehandler.framehandler.07f64a4f-cgu.2.rcgu.o : error LNK2019: unresolved external symbol memmove referenced in function _ZN3std2io8buffered9bufwriter18BufWriter$LT$W$GT$9flush_buf17he7d7626d2d124075E
          framehandler.framehandler.07f64a4f-cgu.2.rcgu.o : error LNK2019: unresolved external symbol __chkstk referenced in function _ZN3std3sys7windows5stdio27write_valid_utf8_to_console17hc2a96813c1090a86E
          framehandler.framehandler.07f64a4f-cgu.2.rcgu.o : error LNK2019: unresolved external symbol memcpy referenced in function _ZN3std10sys_common4wtf87Wtf8Buf25push_code_point_unchecked17hc0fcd0433c9f57d4E
          framehandler.framehandler.07f64a4f-cgu.2.rcgu.o : error LNK2001: unresolved external symbol "const type_info::`vftable'" (??_7type_info@@6B@)
          LINK : error LNK2001: unresolved external symbol mainCRTStartup
          C:\Rust\framehandler\target\release\deps\framehandler.exe : fatal error LNK1120: 10 unresolved externals
```
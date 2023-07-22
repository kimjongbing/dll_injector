# dll_injector
Simple dll injector coded in Rust. If you want to inject a dll into UWP process use my [dll_injector_uwp](https://github.com/kimjongbing/dll_injector_uwp/)


## Usage
1. **dll_injector.exe** ``<list>`` : will list all pids
2. **dll_injector.exe** ``<pid>`` ``<file.exe>`` : will return pid of running exe
3. **dll_injector.exe** ``<pid>`` ``<payload.dll>`` : will inject into process


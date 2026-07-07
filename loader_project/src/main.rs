use libloading::{Library, Symbol};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: loader.exe <module_name>");
        return;
    }
    let target = &args[1];
    println!("[+] Preparing to sideload: {}", target);

    let lib = unsafe { Library::new("RustDoLL.dll").expect("Could not load RustDoLL.dll") };
    type ExecuteCommand = unsafe extern "system" fn(*const u16);

    let exec: Symbol<ExecuteCommand> = unsafe { 
        lib.get(b"ExecuteCommand").expect("Could not find function 'ExecuteCommand'") 
    };

    let cmd = OsStr::new(target).encode_wide().chain(std::iter::once(0)).collect::<Vec<u16>>();

    unsafe {
        println!("[+] Sending instruction to DLL...");
        exec(cmd.as_ptr());
        println!("[+] Execution finished.");
    }
}

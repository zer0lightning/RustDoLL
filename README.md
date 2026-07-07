# RustDoLL: DLL Sideloading Research

A modular Windows project designed for educational research into **DLL Sideloading** and dynamic library injection.

![RustDoLL Banner](assets/banner.png)

## Educational Purpose

This project provides a sandbox for understanding:

* **Dynamic Linking:** How Windows maps external libraries into process memory.
* **Export Table Resolution:** How host applications locate and execute functions inside a loaded DLL.
* **Standalone Testing:** How the Windows loader treats a DLL as a modular, self-executing component.
* **Process Context:** How injected code inherits the permissions and environment of the host process.

## Project Source Structure

The project utilizes a Rust workspace to maintain strict separation between the payload library and the test harness:

```text
RustDoLL/
├── Cargo.toml              # Workspace configuration
├── README.md               # Project documentation
├── dll_project/            # Core library (The Payload)
│   ├── Cargo.toml          # DLL-specific dependencies
│   └── src/
│       └── lib.rs          # Export definitions (ExecuteCommand, Calc, etc.)
└── loader_project/         # Test harness (The Host)
    ├── Cargo.toml          # Loader-specific dependencies
    └── src/
        └── main.rs         # Logic to LoadLibrary and execute exports

```

## Build Instructions

1. **Clone the repository.**
2. **Option 1: Build the DLL and Loader:**
```bash
cd RustDoLL
cargo build --release

```
Output
```
C:\Users\Builder\RustDoLL>cargo build --release
   Compiling loader v0.1.0 (C:\Users\Builder\RustDoLL\loader_project)
   Compiling rustdoll v0.1.0 (C:\Users\Builder\RustDoLL\dll_project)
    Finished `release` profile [optimized] target(s) in 1.36s
```

3. **Option 2: Build the Loader or DLL Only:**
```bash
cd ../loader_project
cargo build --release

```

```bash
cd ../dll_project
cargo build --release

```

**Loader and DLL Compiled Location**
```
C:\Users\Builder\RustDoLL\target\release
```

## Customization for VisitURL and CheckIn Module via Webhook.site
To customize the behavior of the DLL before compiling:

### 1. Networking Endpoints
Open `dll_project/src/lib.rs` and locate the `visit_url_logic` and `check_in_logic` functions. Update the URL strings as follows:

```rust
// In lib.rs
fn visit_url_logic() {
    let url = "https://your-target-url.com"; // UPDATE THIS
    // ...
}

fn check_in_logic() {
    let url = "https://your-webhook-endpoint.com]"; // UPDATE THIS
    // ...
}
```

## Usage

### Method 1: Standalone Testing (`rundll32`)

You can test the functionality of `RustDoLL.dll` as a standalone unit using the native Windows `rundll32.exe` utility.

```cmd
rundll32.exe dll_project\target\release\RustDoLL.dll,<function_name>

```

*Example:* `rundll32.exe RustDoLL.dll,Popup`

![RustDoLL RunDLL32-SideLoad](assets/RunDLL32-SideLoad-Calc.png)

### Method 2: Simulated Sideloading (`loader`)

This method demonstrates how a host application manually maps the DLL and resolves the `ExecuteCommand` entry point to pass dynamic instructions.

* **Run:** `loader.exe <module_name>`
* **Example:** `loader.exe Calc`

![Loader-SideLoad](assets/Loader-Calc.png)

## Payload Modules Reference

| Module | Description |
| --- | --- |
| **`ExecuteCommand`** | Universal entry point; accepts string parameters via memory pointer. |
| **`Calc`** | Spawns the Windows Calculator (`calc.exe`). |
| **`Cmd`** | Spawns a new Command Prompt (`cmd.exe`). |
| **`Powershell`** | Spawns a new PowerShell instance. |
| **`Notepad`** | Launches Notepad (`notepad.exe`). |
| **`Popup`** | Displays a native Windows "Info" message box. |
| **`VisitUrl`** | Triggers a remote HTTP GET request for network beaconing study. |
| **`CheckIn`** | Fingerprints system data (User/Host) for situational awareness. |
| **`RunAll`** | Sequential execution of `Calc` and `Notepad` payloads. |

## Disclaimer

*This project is intended for educational and security research purposes only. Use only on systems you own or have explicit authorization to test. The unauthorized loading of DLLs into processes can be flagged by security software as malicious behavior.*

## Referrence
- https://infosecwriteups.com/dll-search-order-hijacking-finding-and-exploiting-the-flaw-9f5dabaa2470

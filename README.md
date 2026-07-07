# RustDoLL: DLL Sideloading Research

A modular Windows project designed for educational research into **DLL Sideloading** and dynamic library injection.

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
2. **Build the DLL:**
```bash
cd dll_project
cargo build --release

```


3. **Build the Loader:**
```bash
cd ../loader_project
cargo build --release

```



## Usage

### Method 1: Standalone Testing (`rundll32`)

You can test the functionality of `RustDoLL.dll` as a standalone unit using the native Windows `rundll32.exe` utility.

```cmd
rundll32.exe dll_project\target\release\RustDoLL.dll,<function_name>

```

*Example:* `rundll32.exe RustDoLL.dll,Popup`

### Method 2: Simulated Sideloading (`loader`)

This method demonstrates how a host application manually maps the DLL and resolves the `ExecuteCommand` entry point to pass dynamic instructions.

* **Run:** `loader.exe <module_name>`
* **Example:** `loader.exe Calc`

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

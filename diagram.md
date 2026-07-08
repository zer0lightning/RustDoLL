## Execution & Loading Flow Diagrams
### 1. Proxy Execution via Native Tool (rundll32.exe)

When you run rundll32.exe path/to/RustDoLL.dll,Calc, you are explicitly forcing a signed system binary to process an arbitrary file. Note the required function signature constraint to avoid a process crash.


```text
                                  +---------------------------------------+
                                  |             Command Line              |
                                  | "rundll32.exe RustDoLL.dll,Calc"      |
                                  +-----------|---------------------------+
                                              |
                                              v
+---------------------------------------------------------------------------------------+
| SIGNED SYSTEM CONTEXT                                                                 |
|                                                                                       |
|   [ C:\Windows\System32\rundll32.exe ]                                                |
|                                                                                       |
|   1. Parses arguments to isolate DLL string path and entrypoint name ("Calc").        |
|   2. Programmatically invokes LoadLibrary("RustDoLL.dll") to map it.                  |
+------------------------------------------|--------------------------------------------+
                                           |
                                           v
+---------------------------------------------------------------------------------------+
| TARGET FUNCTION INVOKATION (Stack Alignment Critical)                                 |
|                                                                                       |
|   3. rundll32.exe runs GetProcAddress(hModule, "Calc").                               |
|   4. Pushes 4 standard arguments onto the stack and jumps to the symbol pointer.      |
|                                                                                       |
|      Memory Space (rundll32.exe)                                                      |
|      +--------------------------------------------------------+                       |
|      | RustDoLL.dll                                           |                       |
|      |                                                        |                       |
|      |  pub extern "system" fn Calc(                          |                       |
|      |      hwnd, hinst, cmd_line, cmd_show                   |                       |
|      |  ) <--------------------------------- [Aligned Call]   |                       |
|      +--------------------------------------------------------+                       |
+---------------------------------------------------------------------------------------+

```
### 2. Normal Loading via Test Harness (`loader.exe`)

This diagram tracks how your custom loader explicitly targets, maps, and executes the exports from an absolute or relative path.

```text
+-----------------------------------------------------------------+
|                       CUSTOM HOST PROCESS                       |
|                                                                 |
|   [ loader.exe ]                                                |
|         |                                                       |
|         v                                                       |
|   1. Calls API: LoadLibraryW("target/release/RustDoLL.dll")     |
+---------|-------------------------------------------------------+
          |
          v
+-----------------------------------------------------------------+
|                    WINDOWS LOADER (OS KERNEL)                   |
|                                                                 |
|   2. Pinpoints the file explicitly via the provided path.       |
|   3. Maps PE headers & sections into the process memory space.  |
|   4. Calls DllMain() invoking DLL_PROCESS_ATTACH.               |
+---------|-------------------------------------------------------+
          |
          v
+-----------------------------------------------------------------+
|               EXPORT RESOLUTION & PAYLOAD RUN                   |
|                                                                 |
|   5. loader.exe calls: GetProcAddress(hModule, "Calc")          |
|   6. Traverses Export Address Table (EAT) of RustDoLL.dll       |
|   7. Jumps to function pointer and executes.                    |
|                                                                 |
|      Memory Space (loader.exe)                                  |
|      +-----------------------------------------------------+    |
|      | Loaded Module: RustDoLL.dll                         |    |
|      |  -> DllMain()         [Runs on attach]              |    |
|      |  -> Calc()            [Launches calc.exe] <-- Run   |    |
|      +-----------------------------------------------------+    |
+-----------------------------------------------------------------+
```

### 3. Classic DLL Sideloading (Search Order Abuse)

In a true sideloading scenario, the threat model relies on omitting paths. A legitimate application is coerced into loading a custom library because it looks for dependencies in its immediate directory first.

```text
+-------------------------------------------------------------------------------------------+
| DISK LAYOUT (Target Application Folder)                                                   |
|                                                                                           |
|    C:\Program Files\LegitApp\                                                             |
|       ├── LegitExecutable.exe   <-- Legitimate, Digitally Signed Application              |
|       └── version.dll           <-- Your RustDoLL.dll renamed to match expected dependency|
+------------------------------------------|------------------------------------------------+
                                           |
                                           v User launches LegitExecutable.exe
+-----------------------------------------------------------------------------------------+
| WINDOWS LOADER SEARCH ORDER MECHANISM                                                   |
|                                                                                         |
|   1. Application starts and requests a generic dependency: LoadLibraryW("version.dll")  |
|   2. Windows checks standard Search Order:                                              |
|          Position 1: The Directory from which the application loaded. (MATCH FOUND)     |
|          Position 2: System directories (C:\Windows\System32 - Dropped version bypassed)|
+------------------------------------------|----------------------------------------------+
                                           |
                                           v
+----------------------------------------------------------------------------------------+
| EXECUTION CONTEXT                                                                      |
|                                                                                        |
|   3. System maps your custom version.dll into LegitExecutable.exe.                     |
|   4. DllMain() triggers automatically upon mapping.                                    |
|   5. Payload executes entirely inside the trusted process scope of LegitExecutable.exe.|
+----------------------------------------------------------------------------------------+
```

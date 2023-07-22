# dll_injector
A simple DLL injector coded in Rust. This tool is designed for regular Windows applications. If you need to inject a DLL into a UWP process, consider using my [dll_injector_uwp](https://github.com/kimjongbing/dll_injector_uwp/).

## Usage
This tool can be used to list all process IDs (PIDs), retrieve the PID of a running executable, and to inject a DLL into a process. Here's how to use each feature:

1. **List all processes**: To list all PIDs, run the following command:
    ```bash
    dll_injector.exe list
    ```
    This command will output a list of all PIDs currently running on your system.

2. **Get PID of a running executable**: If you know the name of a running executable, you can retrieve its PID by using:
    ```bash
    dll_injector.exe pid <file.exe>
    ```
    Replace `<file.exe>` with the name of the running executable. This command will output the PID of the specified executable.

    Example:
    ```bash
    dll_injector.exe pid notepad.exe
    ```

3. **Inject a DLL into a process**: To inject a DLL into a process, you need the PID of the target process and the path to the DLL you want to inject. Here's the command to use:
    ```bash
    dll_injector.exe <pid> <payload.dll>
    ```
    Replace `<pid>` with the PID of the target process and `<payload.dll>` with the path to the DLL you wish to inject.

    Example:
    ```bash
    dll_injector.exe 1234 C:/path/to/your/payload.dll
    ```

This is a command-line application, so all inputs (arguments) should be passed in the command line. Run the command-line as administrator if you have any issues.


## Compile Guide

1. **Clone the repository**: Run the following command in the terminal to download the source code to your local machine.

    ```bash
    git clone https://github.com/kimjongbing/dll_injector
    ```

2. **Navigate to the project folder**: Change the current directory to the project's root folder by running:

    ```bash
    cd dll_injector-main
    ```

3. **Compile the project**: Build the project in release mode by running the following command in the root directory of the project.

    ```bash
    cargo build --release
    ```

4. **Navigate to the build output**: Change directory to where the built executable is located by running:

    ```bash
    cd ./target/release
    ```

5. **Run the executable**: Start the application by running `dll_injector.exe`. Upon execution, the console will display usage instructions. You can then provide the necessary arguments based on what you want to do.

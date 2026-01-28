# Installation

## 1. Recommended: Download Binary

The easiest way to install `makefile2doc` is to download the latest executable for your operating system.

1. Go to the **[GitHub Releases page](https://github.com/Merlin-Clos/makefile2doc/releases/latest)**.
2. Download the file matching your OS and architecture (e.g., `makefile2doc-linux-amd64`).
3. Follow the setup instructions below for your operating system.

## 2. Setup

### Linux & macOS
To use the command globally, make it executable and move it to a folder in your system `$PATH`.

*Assuming you are in the folder where you downloaded the file:*

```bash
# 1. Give execution permissions
chmod +x makefile2doc-*

# Note: Ensure you only have one version of the file in the folder to avoid errors.
sudo mv makefile2doc-* /usr/local/bin/makefile2doc
```

### Windows
1. Create a folder for your CLI tools (e.g., `C:\Tools`)
2. Move the downloaded file (e.g., `makefile2doc-windows-amd64.exe`) into this folder.
3. Rename the file to `makefile2doc.exe`.
4. Add this folder to your **PATH** environment variable:
    * Search “Env” in the Start Menu
    * Open **Edit the system environment variables**
    * Select **Path** → **Edit** → **New**
    * Paste the path to your folder

## 3. Verify Installation
Open a new terminal and verify that the tool is correctly installed:
```bash
makefile2doc --help
```
If you see the help menu, you are ready to use it!

## 4. Developer Installation (via Cargo)
If you already have Rust and Cargo installed on your machine, you can install the tool directly from the source code.
```bash
cargo install --git https://github.com/Merlin-Clos/makefile2doc --locked
```
This will compile the project and place the binary in your `~/.cargo/bin` folder.
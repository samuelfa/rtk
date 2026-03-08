# Integrating RTK with Gemini CLI

RTK (Rust Token Killer) can be integrated into Gemini CLI to significantly reduce token consumption by filtering and compressing command outputs. We provide three implementation variants depending on your performance and environment needs.

## 🚀 1. High-Performance Rust Hook (Recommended)
This is the fastest option, eliminating startup latency by using a native binary.

### Installation
1. Build the hook:
   ```bash
   cd hooks/gemini-cli
   cargo build --release
   ```
2. Configure `~/.gemini/settings.json`:
   ```json
   "command": "<PATH_TO_RTK_REPO>/hooks/gemini-cli/target/release/rtk-wrapper-rs"
   ```

## 📦 2. Node.js Hook (No Compilation)
Best for users who want a quick setup without building binaries.

### Installation
1. Locate the script at `hooks/gemini-cli/rtk_wrap.js`.
2. Configure `~/.gemini/settings.json`:
   ```json
   "command": "node <PATH_TO_RTK_REPO>/hooks/gemini-cli/rtk_wrap.js"
   ```

## 🐚 3. Bash Hook (Lightweight)
A lightweight shell script alternative. Requires `jq` to be installed on your system.

### Installation
1. Locate the script at `hooks/gemini-cli/rtk-rewrite.sh`.
2. Ensure it is executable: `chmod +x hooks/gemini-cli/rtk-rewrite.sh`
3. Configure `~/.gemini/settings.json`:
   ```json
   "command": "/bin/bash <PATH_TO_RTK_REPO>/hooks/gemini-cli/rtk-rewrite.sh"
   ```

## Comparison
| Variant | Latency | Dependencies | Best For |
| :--- | :--- | :--- | :--- |
| **Rust** | **~7ms** | Rust Toolchain (to build) | Production / Power Users |
| **Bash** | **~15ms** | bash, jq | Minimalist environments |
| **Node.js** | **~40ms** | Node.js | Quick setup |

## Verify Integration
Run `git status` in Gemini CLI. Output should be compact. Check your total gains with:
```bash
rtk gain
```

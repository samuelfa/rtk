# Integrating RTK with Gemini CLI

RTK (Rust Token Killer) can be integrated into Gemini CLI to significantly reduce token consumption by filtering and compressing command outputs.

## Prerequisites
- **RTK Binary**: Ensure you have the main `rtk` binary installed and available in your PATH.
  ```bash
  cargo install rtk
  ```
- **Rust Toolchain**: Required to build the high-performance hook.

## Installation Steps

### 1. Build the High-Performance Hook
Navigate to the Gemini CLI hook directory in this repository and build it:

```bash
cd hooks/gemini-cli
cargo build --release
```

### 2. Configure Gemini CLI
Add the RTK hook to your `~/.gemini/settings.json` file. Replace `<PATH_TO_RTK_REPO>` with the absolute path where you cloned this repository.

```json
{
  "hooks": {
    "BeforeTool": [
      {
        "matcher": "run_shell_command",
        "hooks": [
          {
            "name": "rtk-wrapper",
            "type": "command",
            "command": "<PATH_TO_RTK_REPO>/hooks/gemini-cli/target/release/rtk-wrapper-rs"
          }
        ]
      }
    ]
  }
}
```

## Verify Integration
Run a command in Gemini CLI (e.g., `git status`). The output should be noticeably more compact. You can verify total savings by running:

```bash
rtk gain
```

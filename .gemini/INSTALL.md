# Integrating RTK with Gemini CLI

RTK (Rust Token Killer) can be integrated into Gemini CLI to significantly reduce token consumption by filtering and compressing command outputs.

## Prerequisites
- **RTK Binary**: Ensure you have the main `rtk` binary installed and available in your PATH.
  ```bash
  cargo install --git https://github.com/rtk-ai/rtk
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

## Performance Benchmarks
We have benchmarked the different hook implementations to ensure the best possible experience for Gemini CLI users.

| Implementation | Average Latency | Speed Factor |
| :--- | :--- | :--- |
| **Rust (Native)** | **~7 ms** | **1.0x (Winner)** |
| Bash (+jq) | ~15 ms | 2.2x slower |
| Node.js | ~40 ms | 5.7x slower |

Using the native Rust hook reduces startup latency by over 80% compared to Node.js, resulting in a significantly more responsive CLI experience.

### Running Benchmarks
You can run the included benchmark script to verify performance in your environment:
```bash
python3 hooks/gemini-cli/benchmarks/benchmark_hooks.py
```
*(Note: Requires original scripts to be present at the paths defined in the script)*

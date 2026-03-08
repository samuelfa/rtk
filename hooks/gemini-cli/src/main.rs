
use serde::{Deserialize, Serialize};
use std::io::{self, Read};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct ToolArgs {
    command: Option<String>,
    #[serde(flatten)]
    other: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
struct HookInput {
    #[serde(rename = "toolName")]
    tool_name: String,
    args: ToolArgs,
}

#[derive(Serialize)]
struct HookOutput {
    decision: String,
    #[serde(rename = "hookSpecificOutput")]
    hook_specific_output: HookSpecificOutput,
}

#[derive(Serialize)]
struct HookSpecificOutput {
    tool_input: ToolArgs,
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    if buffer.is_empty() {
        println!("{}", serde_json::json!({ "decision": "allow" }));
        return Ok(());
    }

    let input: HookInput = match serde_json::from_str(&buffer) {
        Ok(val) => val,
        Err(_) => {
            println!("{}", serde_json::json!({ "decision": "allow" }));
            return Ok(());
        }
    };

    if input.tool_name == "run_shell_command" {
        let original_command = input.args.command.clone().unwrap_or_default();
        
        if original_command.is_empty() {
            println!("{}", serde_json::json!({ "decision": "allow" }));
            return Ok(());
        }

        // Try to rewrite using rtk
        let output = Command::new("rtk")
            .arg("rewrite")
            .arg(&original_command)
            .output();

        let rewritten_command = match output {
            Ok(out) if out.status.success() => {
                String::from_utf8_lossy(&out.stdout).trim().to_string()
            }
            _ => original_command,
        };

        let response = HookOutput {
            decision: "allow".to_string(),
            hook_specific_output: HookSpecificOutput {
                tool_input: ToolArgs {
                    command: Some(rewritten_command),
                    other: input.args.other,
                },
            },
        };

        println!("{}", serde_json::to_string(&response).unwrap());
    } else {
        println!("{}", serde_json::json!({ "decision": "allow" }));
    }

    Ok(())
}


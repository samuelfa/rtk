const fs = require('fs');
const { execSync } = require('child_process');

/**
 * RTK Intelligent Hook for Gemini CLI (Node.js version)
 * 
 * This hook delegates command rewrite logic to the RTK binary (Rust),
 * ensuring that only supported commands are optimized for token savings.
 * 
 * Requirements: RTK binary installed and in PATH.
 */

try {
  const inputData = fs.readFileSync(0, 'utf-8');
  if (!inputData) {
    process.stdout.write(JSON.stringify({ decision: "allow" }));
    process.exit(0);
  }

  const input = JSON.parse(inputData);

  if (input.toolName === 'run_shell_command') {
    const originalCommand = (input.args.command || '').trim();
    
    if (!originalCommand) {
      process.stdout.write(JSON.stringify({ decision: "allow" }));
      process.exit(0);
    }

    try {
      // Delegate rewrite logic to RTK binary (rtk rewrite requires version >= 0.23.0)
      const rewritten = execSync(`rtk rewrite ${JSON.stringify(originalCommand)}`, { 
        encoding: 'utf-8',
        stdio: ['ignore', 'pipe', 'ignore'] 
      }).trim();
      
      const response = {
        decision: "allow",
        hookSpecificOutput: {
          tool_input: { ...input.args, command: rewritten }
        }
      };
      process.stdout.write(JSON.stringify(response));
    } catch (e) {
      // Fallback: if rtk rewrite fails or no rewrite is needed, use original command
      process.stdout.write(JSON.stringify({ decision: "allow" }));
    }
  } else {
    process.stdout.write(JSON.stringify({ decision: "allow" }));
  }
} catch (err) {
  process.stdout.write(JSON.stringify({ decision: "allow" }));
}

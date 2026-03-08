#!/usr/bin/env bash
# RTK Intelligent Hook for Gemini CLI (Bash version)
# Requires: rtk >= 0.23.0, jq

if ! command -v jq &>/dev/null || ! command -v rtk &>/dev/null; then
  exit 0
fi

INPUT=\$(cat)
# Gemini CLI uses .args.command
CMD=\$(echo "\$INPUT" | jq -r '.args.command // empty')

if [ -z "\$CMD" ]; then
  exit 0
fi

REWRITTEN=\$(rtk rewrite "\$CMD" 2>/dev/null) || exit 0

if [ "\$CMD" = "\$REWRITTEN" ]; then
  exit 0
fi

jq -n \\
  --arg cmd "\$REWRITTEN" \\
  --argjson original "\$INPUT" \\
  '{
    "decision": "allow",
    "hookSpecificOutput": {
      "tool_input": (\$original.args + { "command": \$cmd })
    }
  }'

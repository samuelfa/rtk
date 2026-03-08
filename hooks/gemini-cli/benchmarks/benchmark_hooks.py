import time
import subprocess
import json
import os

# Rutas de los scripts/binarios
NODE_SCRIPT = '/home/samuel/.gemini/hooks/rtk_wrap.js'
RUST_BINARY = '/home/samuel/.gemini/hooks/rtk-wrapper-rs/target/release/rtk-wrapper-rs'
BASH_SCRIPT = '/home/samuel/ai/rtk/hooks/rtk-rewrite.sh'

# Input JSON de prueba
test_input = {
    "toolName": "run_shell_command",
    "args": {
        "command": "git status"
    }
}
input_str = json.dumps(test_input)

def benchmark(name, cmd_list, iterations=50):
    print(f"🚀 Benchmarking {name}...")
    start_time = time.perf_counter()
    
    for _ in range(iterations):
        proc = subprocess.Popen(
            cmd_list,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        proc.communicate(input=input_str)
        
    end_time = time.perf_counter()
    avg_ms = ((end_time - start_time) / iterations) * 1000
    print(f"✅ {name}: {avg_ms:.2f} ms por llamada\n")
    return avg_ms

# Ejecutar pruebas
print("--- Hook Performance Comparison (50 iterations) ---\n")
results = {}
try:
    results['Node.js'] = benchmark("Node.js", ["node", NODE_SCRIPT])
except Exception as e: print(f"Error en Node: {e}")

try:
    results['Rust (Native)'] = benchmark("Rust", [RUST_BINARY])
except Exception as e: print(f"Error en Rust: {e}")

try:
    results['Bash (+jq)'] = benchmark("Bash", [BASH_SCRIPT])
except Exception as e: print(f"Error en Bash: {e}")

# Conclusión
if results:
    best = min(results, key=results.get)
    print(f"🏆 Ganador: {best}")

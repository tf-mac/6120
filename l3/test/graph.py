#!/usr/bin/env python3

import subprocess
import glob
import matplotlib.pyplot as plt
import numpy as np

def run_cmd(cmd):
    try:
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
        return result.stdout.strip()
    except:
        return "0"

def get_line_count(cmd):
    return int(run_cmd(cmd + " | wc -l"))

def get_dynamic_count(cmd):
    output = run_cmd(cmd + " | brili -p")
    for line in output.split('\n'):
        if 'total_dyn_inst:' in line:
            return int(line.split(':')[1].strip())
    return 0

# Find all .bril files in current directory
bril_files = glob.glob("*.bril")
print(f"Found {len(bril_files)} .bril files")

benchmarks = []
base_static = []
dce_static = []
lvn_static = []
base_dynamic = []
dce_dynamic = []
lvn_dynamic = []

for f in bril_files:
    name = f.replace('.bril', '')
    benchmarks.append(name)
    
    print(f"Processing {name}...")
    
    # Static counts - convert back to bril and then to json for fair comparison
    base_static.append(get_line_count(f"bril2json < {f}"))
    dce_static.append(get_line_count(f"bril2json < {f} | /home/tfm/6120/l3/target/debug/l3 dce | bril2txt | bril2json"))
    lvn_static.append(get_line_count(f"bril2json < {f} | /home/tfm/6120/l3/target/debug/l3 lvn | bril2txt | bril2json"))
    
    # Dynamic counts  
    base_dynamic.append(get_dynamic_count(f"bril2json < {f}"))
    dce_dynamic.append(get_dynamic_count(f"bril2json < {f} | /home/tfm/6120/l3/target/debug/l3 dce"))
    lvn_dynamic.append(get_dynamic_count(f"bril2json < {f} | /home/tfm/6120/l3/target/debug/l3 lvn"))

# Plot
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))

x = np.arange(len(benchmarks))
width = 0.25

# Static
ax1.bar(x - width, base_static, width, label='Base', alpha=0.7)
ax1.bar(x, dce_static, width, label='DCE', alpha=0.7)  
ax1.bar(x + width, lvn_static, width, label='LVN', alpha=0.7)
ax1.set_title('Static Instructions')
ax1.set_ylabel('Line Count')
ax1.legend()
ax1.set_xticks([])

# Dynamic
ax2.bar(x - width, base_dynamic, width, label='Base', alpha=0.7)
ax2.bar(x, dce_dynamic, width, label='DCE', alpha=0.7)
ax2.bar(x + width, lvn_dynamic, width, label='LVN', alpha=0.7)  
ax2.set_title('Dynamic Instructions')
ax2.set_ylabel('Instruction Count')
ax2.legend()
ax2.set_xticks([])

plt.tight_layout()
plt.savefig('optimization_results.png')
plt.show()

print("Done! Results saved to optimization_results.png")

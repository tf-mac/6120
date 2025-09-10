import json
import sys

def prefix(data):
    for func in data['functions']:
        for instr in func['instrs']:
            if 'op' in instr and instr['op'] == 'print':
                for i in range(len(instr['args'])):
                    instr['args'][i] = sys.argv[2] + instr['args'][i]

    return data


if __name__ == "__main__":
    with open(sys.argv[1], 'r') as data:
        out = prefix(json.load(data))
        
    with open(sys.argv[1], 'w') as data:
        json.dump(out, data)

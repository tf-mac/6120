import json
import sys

def build_blocks(body):
    block = []

    for instr in body:
        if 'op' in instr:
            block.append(instr)

            if instr['op'] in ['jmp', 'br', 'ret']:
                yield block
        else:
            yield block

def map_blocks(blocks):
    map = {}

    for block in blocks:
        if 'label' in block[0]:
            name = block[0]['label']
            block = block[1:]
        else:
            name = 'b{}'.format(len(map))

        map[name] = block
    return map

def build_cfg(map):
    out = {}
    for name, block in map.items():
        last = block[-1]

        if last['op'] in ['jmp', 'br']:
            succ = last['labels']
        elif last['op'] == 'ret':
            succ = []
        else:
            if list(map.keys()).index(name) + 1 == len(list(map.keys())):
                succ = []
            else:
                succ = [list(map.keys())[list(map.keys()).index(name) + 1]]
        out[name] = succ
    
    return out

if __name__ == "__main__":
    with json.load(sys.argv[0]) as data:
        for function in data['functions']:
            blocks = build_blocks(function['instrs'])
            map = map_blocks(blocks)
            cfg = build_cfg(map)
            print(cfg)

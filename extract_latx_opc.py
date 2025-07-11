import json
# extract decoding masks from binutils
opcodes = set()
with open('loongarch-opc.c', 'r') as f:
    for line in f:
        parts = line.strip().split(',')
        if len(parts[0]) > 1 and parts[0][0] == '{' and parts[0][2] != '&':
            match_value = int(parts[0][1:], 16)
            match_mask = int(parts[1][1:], 16)
            name = parts[2].strip()[1:-1]
            if match_value == 0:
                continue
            opcodes.add(match_value)

# extract opcode from latx
with open('latx-opc.json', 'r') as f:
    data = json.load(f)
    for inst in data:
        opc = int(data[inst]['opcode'], 16)
        if opc in opcodes or opc == 0 or opc < 0:
            continue
        print(f"0x{opc:08x} {inst}")

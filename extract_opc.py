# extract decoding masks from binutils
with open('loongarch-opc.c', 'r') as f:
    with open('src/opcode.rs', 'w') as out:
        print('pub const OPCODES: &[(u32, u32)] = &[', file=out)
        lines = []
        for line in f:
            parts = line.strip().split(',')
            if len(parts[0]) > 1 and parts[0][0] == '{' and parts[0][2] != '&':
                match_value = int(parts[0][1:], 16)
                match_mask = int(parts[1][1:], 16)
                name = parts[2].strip()[1:-1]
                if match_value == 0:
                    continue
                lines.append(f'    (0x{match_value:08x}, 0x{match_mask:x}), // {name}')
        lines = sorted(lines)
        print('\n'.join(lines), file=out)
        print('];', file=out)

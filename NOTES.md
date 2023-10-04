0x01147xxx: frecip
0000 0001 0001 0100 0111 01 rj rd frecip.s
0000 0001 0001 0100 0111 10 rj rd frecip.d

0x01148xxx: frsqrt
0000 0001 0001 0100 1000 01 rj rd frsqrt.s
0000 0001 0001 0100 1000 10 rj rd frsqrt.d

0x0114cxxx: movgr2fcsr
0000 0001 0001 0100 1100 00 fcsr rd movgr2fcsr
binutils limits fcsr to 2 bits, instead of 5 bits

0x0114cxxx: movfcsr2gr
0000 0001 0001 0100 1100 10 fcsr rd movfcsr2gr
binutils limits fcsr to 2 bits, instead of 5 bits

0x3857xxxx: memory
0011 1000 0101 0111 0 xxxxx rj rd ?
0011 1000 0101 0111 1 000xx rj rd ?

0x385[8-b]xxxx: memory
0011 1000 0101 10x0 0 xxxxx rj rd ld.b
0011 1000 0101 10x0 1 xxxxx rj rd ld.h
0011 1000 0101 10x1 0 xxxxx rj rd ld.w
0011 1000 0101 10x1 1 xxxxx rj rd ld.d

0x385[c-f]xxxx: memory
0011 1000 0101 11x0 0 rk rj rd amswap.b
0011 1000 0101 11x0 1 rk rj rd amswap.h
0011 1000 0101 11x1 0 rk rj rd amadd.b
0011 1000 0101 11x1 1 rk rj rd amadd.h

0x714[45]xxxx: LSX
0111 0001 0100 0100 1 rk rj rd vf?.s
0111 0001 0100 0101 0 rk rj rd vf?.d

0x729bxxxx: LSX
0111 0010 1001 1011 1 imm10 rd vindex.w
vindex.w imm10=0: 0302010003020100, 0302010003020100
vindex.w imm10=1: 0403020103020100, 0605040305040302

0x729d2xxx: vfrsqrt
0111 0010 1001 1101 0010 01 rj rd vfrsqrt.s
0111 0010 1001 1101 0010 10 rj rd vfrsqrt.d

0x754[45]xxxx: LASX
0111 0101 0100 0100 1 rk rj rd xvf?.s
0111 0101 0100 0101 0 rk rj rd xvf?.d

0x769bxxxx: LASX
0111 0110 1001 1011 1 imm10 rd xvindex.w
xvindex.w imm10=0: 0302010003020100, 0302010003020100, 0302010003020100, 0302010003020100
xvindex.w imm10=1: 0403020103020100, 0605040305040302, 0403020103020100, 0605040305040302

0x769dxxxx: xvfrsqrt
0111 0110 1001 1101 0010 01 rj rd xvfrsqrt.s
0111 0110 1001 1101 0010 10 rj rd xvfrsqrt.d
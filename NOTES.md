## Undocumented instructions in LS3A6000

0x0114cxxx: movgr2fcsr
0000 0001 0001 0100 1100 00 fcsr rd movgr2fcsr
binutils limits fcsr to 2 bits, instead of 5 bits

0x0114cxxx: movfcsr2gr
0000 0001 0001 0100 1100 10 fcsr rd movfcsr2gr
binutils limits fcsr to 2 bits, instead of 5 bits

0x06493000: ?
0000 0110 0100 1001 0011 0000 0000 0000

0x3857xxxx: memory
0011 1000 0101 0111 0 xxxxx rj rd ?
0011 1000 0101 0111 1 000xx rj rd ?

0x714[45]xxxx: LSX
0111 0001 0100 0100 1 rk rj rd vf?.s
0111 0001 0100 0101 0 rk rj rd vf?.d

0x729bxxxx: LSX
0111 0010 1001 1011 1 imm10 rd vindex.w
vindex.w imm10=0: 0302010003020100, 0302010003020100
vindex.w imm10=1: 0403020103020100, 0605040305040302

0x754[45]xxxx: LASX
0111 0101 0100 0100 1 rk rj rd xvf?.s
0111 0101 0100 0101 0 rk rj rd xvf?.d

0x769bxxxx: LASX
0111 0110 1001 1011 1 imm10 rd xvindex.w
xvindex.w imm10=0: 0302010003020100, 0302010003020100, 0302010003020100, 0302010003020100
xvindex.w imm10=1: 0403020103020100, 0605040305040302, 0403020103020100, 0605040305040302

## Discovered LoongArch v1.1 instructions

0x01147xxx: frecipe.s/d documented in LoongArch v1.1
0000 0001 0001 0100 0111 01 rj rd frecipe.s
0000 0001 0001 0100 0111 10 rj rd frecipe.d

0x01148xxx: frsqrte.s/d documented in LoongArch v1.1
0000 0001 0001 0100 1000 01 rj rd frsqrte.s
0000 0001 0001 0100 1000 10 rj rd frsqrte.d

0x385[8-b]xxxx: amcas{_db}.[bhwd], documented in LoongArch v1.1
0011 1000 0101 1000 0 rk rj rd amcas.b
0011 1000 0101 1000 1 rk rj rd amcas.h
0011 1000 0101 1001 0 rk rj rd amcas.w
0011 1000 0101 1001 1 rk rj rd amcas.d
0011 1000 0101 1010 0 rk rj rd amcas_db.b
0011 1000 0101 1010 1 rk rj rd amcas_db.h
0011 1000 0101 1011 0 rk rj rd amcas_db.w
0011 1000 0101 1011 1 rk rj rd amcas_db.d

0x385[c-f]xxxx: am(swap|add) documented in LoongArch V1.1
0011 1000 0101 1100 0 rk rj rd amswap.b
0011 1000 0101 1100 1 rk rj rd amswap.h
0011 1000 0101 1101 0 rk rj rd amadd.b
0011 1000 0101 1101 1 rk rj rd amadd.h
0011 1000 0101 1110 0 rk rj rd amswap_db.b
0011 1000 0101 1110 1 rk rj rd amswap_db.h
0011 1000 0101 1111 0 rk rj rd amadd_db.b
0011 1000 0101 1111 1 rk rj rd amadd_db.h

0x729d2xxx: vfrecipe.s/d documented in LoongArch v1.1
0111 0010 1001 1101 0001 01 rj rd vfrecipe.s
0111 0010 1001 1101 0001 10 rj rd vfrecipe.d

0x729d2xxx: vfrsqrte.s/d documented in LoongArch v1.1
0111 0010 1001 1101 0010 01 rj rd vfrsqrte.s
0111 0010 1001 1101 0010 10 rj rd vfrsqrte.d

0x769d1xxx: xvfrecipe.s/d documented in LoongArch v1.1
0111 0110 1001 1101 0001 01 rj rd xvfrecipe.s
0111 0110 1001 1101 0001 10 rj rd xvfrecipe.d

0x769d2xxx: xvfrsqrte.s/d documented in LoongArch v1.1
0111 0110 1001 1101 0010 01 rj rd xvfrsqrte.s
0111 0110 1001 1101 0010 10 rj rd xvfrsqrte.d

## LoongArch v1.1

- 之前已发现：新增近似求解浮点数开根和浮点数开根求倒数指令，包括标量运算的 FRECIPE.S、FRECIPE.D、FRSQRTE.S、FRSQRTE.D 指令，128 位 SIMD 运算的 VFRECIPE.S、VFRECIPE.D、VFRSQRTE.S、VFRSQRTE.D 指令和 256 位 SIMD 运算的 XVFRECIPE.S、XVFRECIPE.D、XVFRSQRTE.S、 XVFRSQRTE.D 指令。
- 之前未发现：新增 SC.Q 指令。
- 之前未发现：新增 LLACQ.W、SCREL.W、LLACQ.D、SCREL.D 指令。
- 之前已发现，判断为 LD，实际上是 AMCAS：新增 AMCAS.B、AMCAS.H、AMCAS.W、AMCAS.D、AMCAS_DB.B、AMCAS_DB.H、AMCAS_DB.W、AMCAS_DB.D 指令。
- 之前已发现：新增 AMADD.B、AMADD.H、AMSWAP.B、AMSWAP.H、AMSWAP_DB.B、AMSWAP_DB.H、AMADD_DB.B、AMADD_DB.H 指令。
- 从近期 GCC/LLVM 变更中已发现：增加 dbar 指令部分非零 hint 值的功能定义。
- 3A5000 遗留问题：新增 64 位机器上执行 32 位整数除法指令是否受源操作数寄存器高 32 位值影响的判定方式。
- 之前未发现：规范相同地址 load 访存操作顺序执行行为判定方式。
- 之前未发现：增加消息中断的定义。
- 之前未发现：允许实现硬件页表遍历。

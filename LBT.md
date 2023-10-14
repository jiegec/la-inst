# LBT

## registers

5 registers: LBT0 to LBT4

LBT0 to 3 for scratch,

LBT4: high 32 bits ftop, low 32 bits eflags

## scratch

scratch registers:

- $scr0: LBT0
- $scr1: LBT1
- $scr2: LBT2
- $scr3: LBT3

```c
// movgr2scr $scrd, $rj
movgr2scr(scrd, rj) {
    SCR[scrd] = GPR[rj];
}

// movscr2gr $rd, $scrj
movgr2scr(rd, scrj) {
    SCR[rd] = GPR[scrj];
}
```

### jiscr

- jiscr0 imm
- jiscr1 imm

```c
// jiscr0 imm
jiscr0(imm) {
    PC = SCR[0] + imm;
}

// jiscr1 imm
jiscr1(imm) {
    PC = SCR[1] + imm;
}
```


## x86

### ftop

- x86mttop imm
- x86mftop rd
- x86inctop
- x86dectop

ftop = floating point stack top pointer

Intel x87 FPU: eigth level deep stack

```c
// x86mttop imm
x86mttop(imm) {
    TOP = imm;
}

// x86mftop $rd
x86mftop(rd) {
    GPR[rd] = TOP;
}

// x86inctop
x86inctop() {
    TOP = (TOP + 1) & 0x7;
}

// x86dectop
x86dectop() {
    TOP = (TOP - 1) & 0x7;
}
```

linux lbt.S:

```
/*
 * a0: ftop
 */
SYM_FUNC_START(_save_ftop_context)
    x86mftop    t1
    st.w        t1, a0, 0
    li.w        a0, 0           # success
    jr      ra
SYM_FUNC_END(_save_ftop_context)

/*
 * a0: ftop
 */
SYM_FUNC_START(_restore_ftop_context)
    ld.w        t1, a0, 0
    andi        t1, t1, 0x7
    la.pcrel    a0, 1f
    alsl.d      a0, t1, a0, 3
    jr      a0
1:
    x86mttop    0
    b   2f
    x86mttop    1
    b   2f
    x86mttop    2
    b   2f
    x86mttop    3
    b   2f
    x86mttop    4
    b   2f
    x86mttop    5
    b   2f
    x86mttop    6
    b   2f
    x86mttop    7
2:
    li.w        a0, 0           # success
    jr      ra
SYM_FUNC_END(_restore_ftop_context)
```

### setloop

- setx86loope rd, rj
- setx86loopne rd, rj

conditional loop according to rj and EFLAGS?

### inc/dec

- x86inc.b/h/w/d rj
- x86dec.b/h/w/d rj

LBT4 changed (eflags), GPR unchanged

### tm

- x86settm
- x86clrtm

tm = floating point stack mode

related to x87 FPU as well?

### setj

- setx86j rd, imm

j = jump? conditional jump by comparing to EFLAGS?

### mul/add/sub/adc/sbc/sll/sra/rotr/rotl/rcr/rcl/and/or/xor

all accept rj, rk args

LBT4 changed (eflags), GPR unchanged

### slli/srli/srai/rotri/rcri/rotli/rcli

all accept rj, imm args

LBT4 changed (eflags), GPR unchanged

### settag

- x86settag rd, imm1, imm2

some computation based on imm1, imm2 and EFLAGS?

### flag

- x86mfflag rd, mask: read from EFLAGS
- x86mtflag rd, mask: write to EFLAGS

kernel lbt.S:

```
    ldptr.d     t1, a0, THREAD_EFLAGS   # restore eflags
    x86mtflag   t1, 0x3f

    x86mfflag   t1, 0x3f        # save eflags
    EX  st.w    t1, a1, 0
```

### long double

- fcvt.ld.d
- fcvt.ud.d
- fcvt.d.ld

for x87 80-bit extended precision

## arm

### move

- armmove rd, rj, imm

TODO

### setj

- setarmj rd, imm

TODO

### add/sub/adc/sbc/and/or/xor/sll/srl/sra/rotr

- armadd.w rd, rj, imm

writes LBT4 (FLAGS), GPR unchanged

### slli/srli/srai/rotri

- armslli.w rd, imm1, imm2

writes LBT4 (FLAGS), GPR unchanged

### not

- armnot.w rd, imm

writes LBT4 (FLAGS), GPR unchanged

### mov

- armmov.w/d rd, imm

writes LBT4 (FLAGS), GPR unchanged

### rrx

- armrrx.w rd, imm

writes LBT4 (FLAGS), GPR unchanged

### fflag

- armmfflag rd, imm
- armmtflag rd, imm

## mips

### ldl/ldr/stl/str

for MIPS style unaligned load/stores

## references

https://web.archive.org/web/20190713073150/http://www.loongson.cn/uploadfile/cpumanual/LoongsonGS264_user.pdf
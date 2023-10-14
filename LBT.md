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

### eflags

```c
CF = (EFLAGS & 0x001) != 0;
PF = (EFLAGS & 0x004) != 0;
AF = (EFLAGS & 0x010) != 0;
ZF = (EFLAGS & 0x040) != 0;
SF = (EFLAGS & 0x080) != 0;
OF = (EFLAGS & 0x800) != 0;
```

All set: 0x8d5

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

```asm
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

### setloope/setloopne

- setx86loope rd, rj
- setx86loopne rd, rj

match x86 LOOPE and LOOPNE instructions, see https://www.felixcloutier.com/x86/loop:loopcc, thanks @xen0n.

```c
setx86loope(rd, rj) {
    GPR[rd] = GPR[rj] != 0 && ZF == 1;
}

setx86loopne(rd, rj) {
    GPR[rd] = GPR[rj] != 0 && ZF == 0;
}
```

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

According to 龙芯指令系统架构技术:

```asm
# x86
SUB ECX, EDX
JE X86_target

# lbt in paper
SUB.W Result, Recx, Redx    # compute result
X86SUB.W Reflag, Recx, Redx # update eflags
SETX86J Rtmp, Reflag, EQ    # generate jump condition
BNE Rtmp, R0, LA_target     # actual jump

# actual lbt where Reflag is implicit
SUB.W Result, Recx, Redx    # compute result
X86SUB.W Recx, Redx         # update eflags
SETX86J Rtmp, EQ            # generate jump condition
BNE Rtmp, R0, LA_target     # actual jump
```

According to http://unixwiz.net/techtips/x86-jumps.html, x86 jump variants:

- jo: OF=1
- jno: OF=0
- js: SF=1
- jns: SF=0
- je/jz: ZF=1
- jne/jnz: ZF=0
- jb/jnae/jc: CF=1
- jnb/jze/jnc: CF=0
- jbe/jna: CF=1 || ZF=1
- ja/jnbe: CF=0 && ZF=0
- jl/jnge: SF != OF
- jge/jnl: SF == OF
- jle/jng: ZF=1 || SF != OF
- jg/jnle: ZF=0 && SF == OF
- jp/jpe: PF=1
- jnp/jpo: PF=0

```c
// setx86j $rd, imm
setx86j(rd, imm) {
    switch(imm) {
        case 0:
            GPR[rd] = CF == 0 && ZF == 0;
            break;
        case 1:
            GPR[rd] = CF == 0;
            break;
        case 2:
            GPR[rd] = CF == 1;
            break;
        case 3:
            GPR[rd] = CF == 1 || ZF == 1;
            break;
        case 4:
            GPR[rd] = ZF == 1;
            break;
        case 5:
            GPR[rd] = ZF == 0;
            break;
        case 6:
            GPR[rd] = ZF == 0 && SF == OF;
            break;
        case 7:
            GPR[rd] = SF == OF;
            break;
        case 8:
            GPR[rd] = SF != OF;
            break;
        case 9:
            GPR[rd] = ZF == 1 || SF != OF;
            break;
        case 10:
            GPR[rd] = SF == 1;
            break;
        case 11:
            GPR[rd] = SF == 0;
            break;
        case 12:
            GPR[rd] = OF == 1;
            break;
        case 13:
            GPR[rd] = OF == 0;
            break;
        case 14:
            GPR[rd] = PF == 1;
            break;
        case 15:
            GPR[rd] = PF == 0;
            break;
    }
}
```

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

```c
// x86mfflag $rd, imm
x86mfflag(rd, imm) {
    GPR[rd] = 0;
    if ((imm & 0x01) != 0) {
        GPR[rd] |= CF * 0x001;
    }
    if ((imm & 0x02) != 0) {
        GPR[rd] |= PF * 0x004;
    }
    if ((imm & 0x04) != 0) {
        GPR[rd] |= AF * 0x010;
    }
    if ((imm & 0x08) != 0) {
        GPR[rd] |= ZF * 0x040;
    }
    if ((imm & 0x10) != 0) {
        GPR[rd] |= SF * 0x080;
    }
    if ((imm & 0x20) != 0) {
        GPR[rd] |= OF * 0x800;
    }
}

// x86mtflag $rd, imm
x86mtflag(rd, imm) {
    if ((imm & 0x01) != 0) {
        CF = (GPR[rd] & 0x001) != 0;
    }
    if ((imm & 0x02) != 0) {
        PF = (GPR[rd] & 0x004) != 0;
    }
    if ((imm & 0x04) != 0) {
        AF = (GPR[rd] & 0x010) != 0;
    }
    if ((imm & 0x08) != 0) {
        ZF = (GPR[rd] & 0x040) != 0;
    }
    if ((imm & 0x10) != 0) {
        SF = (GPR[rd] & 0x080) != 0;
    }
    if ((imm & 0x20) != 0) {
        OF = (GPR[rd] & 0x800) != 0;
    }
}
```

kernel lbt.S:

```asm
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

- https://web.archive.org/web/20190713073150/http://www.loongson.cn/uploadfile/cpumanual/LoongsonGS264_user.pdf
- 龙芯指令系统架构技术
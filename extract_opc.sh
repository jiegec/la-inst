#!/bin/sh
wget https://raw.githubusercontent.com/bminor/binutils-gdb/master/opcodes/loongarch-opc.c -O loongarch-opc.c
python3 extract_opc.py

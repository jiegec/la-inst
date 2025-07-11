#!/bin/sh
wget https://github.com/lat-opensource/lat/raw/refs/heads/master/target/i386/latx/inst_template.json -O latx-opc.json
python3 extract_latx_opc.py

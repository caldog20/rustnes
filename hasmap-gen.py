import csv
import os
import sys
import pandas as pd
from collections import defaultdict

I = "Immediate"
Z = "ZeroPage"
ZX = "ZeroPageX"
A = "Absolute"
AX = "AbsoluteX"
AY = "AbsoluteY"
IX = "IndirectX"
IY = "IndirectY"

filename = "oplist.txt"
d = defaultdict(list)
with open(filename, 'r', newline='') as f:
    reader = csv.reader(f, delimiter='\t')
    next(reader) # toss headers
    for ticket, asset in reader:
        d[ticket].append(asset)



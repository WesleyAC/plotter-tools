#!/usr/bin/env python
import sys

# this program converts a HP-GL file that is in only absolute movements to one
# that is in only relative movements. It is very limited - it only works on
# files that have one command per line, don't use `PD` or `PU` to move the pen,
# and only use `PA` movement commands with a single position as a parameter. I
# use it, in addition to some vim macros to convert HP-GL files outputted by
# inkscape to ones that can be used at arbitrary points in a script. It's pretty
# janky right now, improvements are appreciated <3

with open(sys.argv[1]) as f:
    cx = 0
    cy = 0
    for l in f.read().split("\n"):
        if l[:2] == "PA":
            ls = l[2:-1].split(",")
            mx = int(ls[0])
            my = int(ls[1])
            print(f"PR{mx-cx},{my-cy};")
            cx = mx
            cy = my
        else:
            print(l)

import sys

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

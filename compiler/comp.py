b = """012++08+3+81+@"""

right = {
    "0":[r"     _",
         r"\   / ",
         r" |  | ",
         r" | /  ",
         r" ||   ",
         r" | \  ",
         r" |  | ",
         r" \_/  "],

    "1":[r"    _",
         r"\__/ "],

    "2":[r"     _",
         r"\___/ "],

    "3":[r"      _",
         r"\____/ "],

    "4":[r"_       _",
         r" \_____/ "],

    "5":[r"        _",
         r"\______/ "],

    "6":[r"         _",
         r"\_______/ "],

    "7":[r"          _",
         r"\________/ "],

    "8":[r"           _",
         r"\_________/ "],

    "9":[r"            _",
         r"\__________/ "],

    "+":[r"     _",
         r"\  _/ ",
         r"| /   ",
         r"| |   ",
         r"\_/   "],

    "@":["_@",],}

out = [""]

for char in b:
    block = right[char]
    linelen = max([len(ln) for ln in out])
    for i in range(len(out)):
        if len(out[i]) < linelen:
            out[i] += " " * (linelen - len(out[i]))
    
    for i, line in enumerate(block):
        if i >= len(out):
            out.append(" " * linelen)
        out[i] += block[i]

print("\n".join(out))
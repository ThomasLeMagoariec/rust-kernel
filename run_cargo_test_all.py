import os

RESULTS = ""

tests = open("./testlist.txt", "r").read().split("\n")

os.system("cargo test > output.txt")

cmd = open("./output.txt", "r").read()
cmd = cmd.split("`")
os.system(cmd[1] + " > result.txt")

RESULTS += open("./result.txt", "r").read() + "\n"

for test in tests:
    os.system(f"cargo test --test {test} > output.txt")

    cmd = open("./output.txt", "r").read()
    cmd = cmd.split("`")
    os.system(cmd[1] + " > result.txt")
    RESULTS += open("./result.txt", "r").read() + "\n"


os.system("cls")
print(RESULTS)
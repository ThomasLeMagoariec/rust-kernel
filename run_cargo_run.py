import os

os.system("cargo run > output.txt")

cmd = open("./output.txt", "r").read()

cmd = cmd.split("`")


os.system(cmd[1])
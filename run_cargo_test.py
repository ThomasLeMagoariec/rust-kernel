import os

os.system("cargo test > output.txt")

cmd = open("./output.txt", "r").read()

cmd = cmd.split("`")


os.system(cmd[1] + " > result.txt")
os.system("cls")
print(open("./result.txt", "r").read())
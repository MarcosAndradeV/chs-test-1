import os, subprocess

if not os.path.exists("./tmp/chsvm"):
    print("Please use: make chsvm")
    exit(0)

directory = './tests'
for filename in os.listdir(directory):
    if filename.endswith('.chs') and "chsvm" in os.listdir("./tmp"):
            subprocess.run(f"./tmp/chsvm run {directory}/{filename}", shell=True, executable="/usr/sbin/zsh")
            subprocess.run(f"rm {directory}/{filename}.chsb", shell=True, executable="/usr/sbin/zsh")
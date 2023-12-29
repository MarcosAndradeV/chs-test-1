import os, subprocess

directory = './tests'
for filename in os.listdir(directory):
    if filename.endswith('.chs') and "chsvm" in os.listdir("./tmp"):
            subprocess.run(f"./tmp/chsvm run {directory}/{filename}", shell=True, executable="/usr/sbin/zsh")
            subprocess.run(f"rm {directory}/{filename}.chsb", shell=True, executable="/usr/sbin/zsh")
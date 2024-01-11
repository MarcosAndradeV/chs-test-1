import os, subprocess, sys

if not os.path.exists("./tmp/chsc"):
    print("Please use: make chsc")
    exit(0)

if len(sys.argv) > 1 :
     if sys.argv[1] == "retest":
        directory = './tests'
        for filename in os.listdir(directory):
            if filename.endswith('.chs') and "chsc" in os.listdir("./tmp"):
                    out = subprocess.run(f"./tmp/chsc run {directory}/{filename}", shell=True, executable="/usr/sbin/zsh", capture_output=True)
                    with open(f"{directory}/out/{filename}.out.expect", "w", encoding="utf-8") as f: f.write(out.stdout.decode("utf-8"))
        exit(0)


directory = './tests'
for filename in os.listdir(directory):
    if filename.endswith('.chs') and "chsc" in os.listdir("./tmp"):
            out = subprocess.run(f"./tmp/chsc run {directory}/{filename}", shell=True, executable="/usr/sbin/zsh", capture_output=True)
            with open(f"{directory}/out/{filename}.out.expect", "r", encoding="utf-8") as f:
                if f.read() != out.stdout.decode("utf-8"):
                    print(f"{filename} has incompatibilities.")
                    continue
                else: print(f"{filename} has passed.")

import os

files = os.listdir(".")

files = [f for f in files if (not f.endswith("PACK.png")) and f.startswith("S")]

for f in files:
    os.remove(f"./{f}")

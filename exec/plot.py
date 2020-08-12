import matplotlib.pyplot as plt
import json
import subprocess
import os

# Opening executable that will produce the sierpinski coordinates
p = subprocess.Popen('./exec/sierpinski')
p.wait()

# Setting image size
fig = plt.figure(figsize=(70, 70))

# After the Rust executable runs, parse the json data it created
with open("./exec/data.json") as json_file:
	data = json.load(json_file)

	# x and y are respectively a list of each points coordinate
	x = [x for x in data[0]]
	y = [y for y in data[1]]

# Creating graphic representation from the data
plt.scatter(x, y, s=(72./fig.dpi)**2)
fig.savefig("./exec/sierpinski.png")

os.remove("./exec/data.json")
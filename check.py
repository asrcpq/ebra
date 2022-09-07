import sys
import os
path = sys.argv[1]
ids = []
last_epoch = 0
lines = 0
print("hi")
def check_file(path):
	global last_epoch, lines
	for line in open(path):
		epoch = line.split(None, 1)[0]
		epoch = float(epoch)
		assert epoch >= last_epoch
		last_epoch = epoch
		lines += 1

if os.path.exists(f"{path}.ard"):
	for file in os.listdir(f"{path}.ard"):
		split = file.split('.')
		assert split[1] == 'art'
		assert len(split) == 2
		idx = int(split[0])
		assert idx >= 0
		ids.append(idx)
	ids = sorted(ids)
	assert ids[-1] + 1 == len(ids)
	for idx in range(len(ids)):
		check_file(f"{path}.ard/{idx}.art")
check_file(path)

print("{} arts, {} lines, last time {}".format(len(ids), lines, last_epoch))
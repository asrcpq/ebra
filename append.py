#!/usr/bin/python3

import sys
import os

rlen = 10000
dst = sys.argv[1]

def parse_epoch(line):
	idx = 0
	result = 0
	# TODO: float epoch
	while ord(b'0') <= line[idx] and line[idx] <= ord(b'9'):
		result *= 10
		result += line[idx] - ord(b'0')
		idx += 1
		if idx > len(line): # empty msg
			break
	assert result > 0
	return result

def bin_split(b):
	if len(b) == 0:
		return []
	d = b.split(b'\n')
	assert d[-1] == b''
	return d[:-1]

def get_rotcount(dst):
	aorot_dir = dst + ".ard"
	if not os.path.exists(aorot_dir):
		os.mkdir(aorot_dir)
		rotcount = 0
	else:
		ns = []
		for file in os.listdir(aorot_dir):
			if file.endswith(".art"):
				ns.append(int(file.removesuffix(".art")))
		ns = sorted(ns)
		if ns:
			assert len(ns) == ns[-1] + 1
		rotcount = len(ns)
	loaded_rot = rotcount
	print(f"find {rotcount} files")
	return rotcount

dst_len = 0
rotcount = 0
last_epoch = None

print("finding last epoch")
if os.path.exists(dst):
	dst_file = open(dst, "rb")
	data_split = bin_split(dst_file.read())
	dst_len = len(data_split)
	if len(data_split) > 0:
		last_epoch = parse_epoch(data_split[-1])
	else:
		print("File found, but empty")
	dst_file.close()
else:
	print("File not found, create")
	dst_file = open(dst, "wb")
	dst_file.close()

rotcount = get_rotcount(dst)
if not last_epoch:
	if rotcount == 0:
		print("Writing to empty dst")
		last_epoch = -1
	else:
		last_rot_file_path = f"{dst}.ard/{rotcount - 1}.art"
		print(f"Last epoch in file {last_rot_file_path}")
		last_rot = open(last_rot_file_path, "rb").read()
		last_rot = bin_split(last_rot)
		last_epoch = parse_epoch(last_rot[-1])
print(f"Last epoch found {last_epoch}")

dst_file = open(dst, "ab")

skipping = True
data_split = sys.stdin.buffer.read()
data_split = bin_split(data_split)
write_count = 0
for (idx, line) in enumerate(data_split):
	if skipping:
		if parse_epoch(line) <= last_epoch:
			# TODO: strict mode
			continue
		else:
			print(f"Line {idx} is newer than {last_epoch}, start writing")
			skipping = False

	dst_file.write(line + b'\n')
	write_count += 1
	dst_len += 1
	if dst_len >= rlen:
		dst_file.close()
		os.rename(dst, f"{dst}.ard/{rotcount}.art")
		print(f"create new rot file {rotcount}")
		rotcount += 1
		dst_file = open(dst, "ab")
		dst_len = 0

print(f"Finished writing {write_count} lines")
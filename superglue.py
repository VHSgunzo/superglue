#!/usr/bin/env python3
import os
import sys

try:
    file_0_path = sys.argv[1]  # Small ELF binary runtime/Shell script
    file_1_path = sys.argv[2]  # Big SquashFS image/Archive
    if not os.path.isfile(file_0_path) or not os.path.isfile(file_1_path):
        raise FileNotFoundError
except (IndexError, FileNotFoundError):
    print('Specify two files!')
    sys.exit(1)

file_0_size = os.path.getsize(file_0_path)
file_1_size = os.path.getsize(file_1_path)
if not file_0_size <= file_1_size:
    print('It is necessary that the first file is less than or equal to the second!')
    sys.exit(1)

with open(file_0_path, 'r+b') as file_0:
    tmp_data_0 = file_0.read(-1)
    with open(file_1_path, 'r+b') as file_1:
        parts_number = int(file_1_size / file_0_size)
        parts_size = file_0_size * parts_number
        remains_part_size = file_1_size - parts_size
        last_part_size = file_0_size + remains_part_size
        file_1.seek(file_1_size - last_part_size)
        last_part_data = file_1.read(-1)
        file_1.truncate(parts_size)

        seek_size = 0
        file_1.seek(seek_size)
        for _ in range(parts_number):
            tmp_data_1 = file_1.read(file_0_size)
            file_1.seek(seek_size)
            file_1.write(tmp_data_0)
            tmp_data_0 = tmp_data_1
            seek_size += file_0_size
        file_1.seek(seek_size)
        file_1.write(last_part_data)

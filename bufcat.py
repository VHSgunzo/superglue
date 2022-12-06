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

with open(file_0_path, 'rb') as file_0:
    file_0_data = file_0.read(-1)
with open(file_1_path, 'rb') as file_1:
    file_1_data = file_1.read(-1)
with open(file_1_path, 'wb') as file_1:
    file_1.write(file_0_data)
    file_1.write(file_1_data)

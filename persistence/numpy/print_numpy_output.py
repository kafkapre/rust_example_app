import os

import numpy as np

dir_path = "./"
file_name = "data_1.npy"
actual = np.load(os.path.join(dir_path, file_name))
print(actual.dtype)
print(actual)

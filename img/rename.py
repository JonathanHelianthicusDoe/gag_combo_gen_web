#!/usr/bin/env python3

from os import listdir, rename
from os.path import isfile, join


onlyfiles = [join(".", f) for f in listdir(".") if isfile(join(".", f))]
for f in onlyfiles:
    rename(f, f.replace(".png", "_org.png"))

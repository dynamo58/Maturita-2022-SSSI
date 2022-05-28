#!/usr/bin/python

import os, sys
import random

def strip(text):
	return text[0:text.rfind(".")]

r = os.listdir(".")

if "--vše" in sys.argv:
	random.shuffle(r)
	print(", ".join(list(map(strip, r))))
else:
	print(r[random.randint(0, len(r))])

#!/usr/bin/python

import os
import random


r = os.listdir(".")
m = len(r)

print(r[random.randint(0, m)])
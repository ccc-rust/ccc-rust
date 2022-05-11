from math import *
import numpy as np
from fixPoint import fixPoint
# 範例 -- 尋找某縮小地圖的不動點。

def rotate(x, a):
    rx = [cos(a)*x[0] - sin(a)*x[1], 
          sin(a)*x[0] + cos(a)*x[1]]
    return np.array(rx)

def move(x, d):
    return np.add(x,d)

def scale(x, s):
    return x*s

def map(x):
    sx = scale(x, 0.2)
    mx = move(sx, [0.5, 0.5])
    rx = rotate(mx, pi/4)
    return rx

np.set_printoptions(precision=5,suppress=True)
p0 = np.array([0.1, 0.1])
fixPoint(map, p0, np.linalg.norm)

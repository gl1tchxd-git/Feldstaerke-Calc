import array as arr
import math

drahtD = 0.0011
zylinderD = 0.025
zylinderL = 0.23
amperage = 1.8

dl = arr.array("d",[0.0, 0.0, 0.0])
ri = arr.array("d",[0.0, 0.0, 0.0])
dlHalf = arr.array("d", [0.0, 0.0, 0.0])

r = []
addUp = []
r1 = arr.array("d", [0.0, 0.0, 0.0])
x = 10
addUp1 = arr.array("d",[0.0, 0.0, 0.0])
for i in range(x):
    r1[1] = i * zylinderL / x
    r.append(r1)
    addUp.append(addUp1)

kreutzprodukt = arr.array("d",[0.0, 0.0, 0.0])
vectorDifference = arr.array("d",[0.0, 0.0, 0.0])
calculatedSum = arr.array("d",[0.0, 0.0, 0.0])

positionStart = arr.array("d",[0.0, 0.0, (zylinderD + drahtD) / 2])
positionEnd = arr.array("d",[0.0, 0.0, (zylinderD + drahtD) / 2])
calculateSumH = 0
h = []

sectionsPerLoop = 4000
sectionsLength = (zylinderD + drahtD) * math.pi / sectionsPerLoop


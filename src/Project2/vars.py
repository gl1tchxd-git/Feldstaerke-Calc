import array as arr
import math

drahtD = 0.0011
zylinderD = 0.025
zylinderL = 0.23
amperage = 1.8

dl = arr.array("d",[0.0, 0.0, 0.0])
ri = arr.array("d",[0.0, 0.0, 0.0])
dlHalf = arr.array("d", [0.0, 0.0, 0.0])
r = arr.array("d", [0.0, zylinderL / 2, 0.0])

addUp = arr.array("d",[0.0, 0.0, 0.0])
kreutzprodukt = arr.array("d",[0.0, 0.0, 0.0])
vectorDifference = arr.array("d",[0.0, 0.0, 0.0])
calculatedSum = arr.array("d",[0.0, 0.0, 0.0])

positionStart = arr.array("d",[0.0, 0.0, (zylinderD + drahtD) / 2])
positionEnd = arr.array("d",[0.0, 0.0, (zylinderD + drahtD) / 2])
calculateSumH = 0


sectionsPerLoop = 4000
sectionsLength = (zylinderD + drahtD) * math.pi / sectionsPerLoop


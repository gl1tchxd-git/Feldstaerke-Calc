from vars import *
from functions import *

#n = zylinderL / drahtD - 1
n = 100

for i in range(sectionsPerLoop * n):
    changePosition(positionEnd, drahtD, zylinderD, zylinderL, sectionsPerLoop, n)
    subtractVector(positionEnd, positionStart, dl)
    divideVector(dl, 2.0, dlHalf)
    addVector(positionStart, dlHalf, ri)
    changePosition(positionStart, drahtD, zylinderD, zylinderL, sectionsPerLoop, n)
    subtractVector(r, ri, vectorDifference)
    calculateSum(dl, kreutzprodukt, vectorDifference, calculatedSum)
    addVector(addUp, calculatedSum, addUp)

print(positionStart)
h = amperage / (4 * math.pi) * calculateVector(addUp)
print(h, "A/m")
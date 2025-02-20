from vars import *
from functions import *

#n = zylinderL / drahtD - 1
n = 210

for i in range(sectionsPerLoop * n):
    changePosition(positionEnd, drahtD, zylinderD, sectionsPerLoop)
    subtractVector(positionEnd, positionStart, dl)
    divideVector(dl, 2.0, dlHalf)
    addVector(positionStart, dlHalf, ri)
    changePosition(positionStart, drahtD, zylinderD, sectionsPerLoop)
    subtractVector(r, ri, vectorDifference)
    calculateSum(dl, kreutzprodukt, vectorDifference, calculatedSum)
    addVector(addUp, calculatedSum, addUp)

h = amperage / (4 * math.pi) * calculateVector(addUp)
print(h, "A/m")
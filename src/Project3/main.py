from vars import *
from functions import *

#n = zylinderL / drahtD - 1
n = 210

for i in range(x):
    for j in range(sectionsPerLoop * n):
        changePosition(positionEnd, drahtD, zylinderD, sectionsPerLoop)
        subtractVector(positionEnd, positionStart, dl)
        divideVector(dl, 2.0, dlHalf)
        addVector(positionStart, dlHalf, ri)
        changePosition(positionStart, drahtD, zylinderD, sectionsPerLoop)
        subtractVector(r[i], ri, vectorDifference)
        calculateSum(dl, kreutzprodukt, vectorDifference, calculatedSum)
        addVector(addUp[i], calculatedSum, addUp[i])

for j in range(x):
    calculateH = amperage / (4 * math.pi) * calculateVector(addUp[j])
    calculateSumH += calculateH
    h.append(calculateH)

print(calculateSumH, "A/m")
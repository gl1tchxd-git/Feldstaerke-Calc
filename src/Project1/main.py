from vars import *
from functions import *

# rH = float(input("Enter radius H: "))
# amperage = float(input("Enter I: "))

for i in range(sections):
    positionMiddle[2] += sectionLength
    dl = subtractVector(positionMiddle, positionStart)
    positionStart[2] += sectionLength
    positionRi[2] += dl[2] / 2
    ri = subtractVector(positionRi, origin)
    r = subtractVector(positionH, origin)
    addUp = addVector(calculateSum(dl, ri, r), addUp)

h = amperage / (4 * math.pi) * calculateVector(addUp) #* (4e-7 * math.pi) B

print("H = ", h, "A/m")

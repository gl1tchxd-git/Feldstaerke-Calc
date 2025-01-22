from vars import *
from functions import *

for i in range(sections):
    positionMiddle[2] += sectionLength
    dl = subtractVector(positionMiddle, positionStart)
    positionRi[2] += dl[2] / 2
    ri = subtractVector(positionRi, origin)
    r = subtractVector(positionH, origin)
    addUp = addVector(calculateSum(dl, ri, r), addUp)

h = amperage / (4 * math.pi) * calculateVector(addUp)

print("H = ", h, "A/m")

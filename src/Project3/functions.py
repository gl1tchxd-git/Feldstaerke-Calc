import math

def changePosition (positionStart, drahtD, zylinderD, sectionsPerLoop):
    if positionStart[2] > 0:
        positionStart[0] += (drahtD + zylinderD) / sectionsPerLoop
    else:
        positionStart[0] -= (drahtD + zylinderD) / sectionsPerLoop

    positionStart[1] += drahtD / sectionsPerLoop

    if positionStart[0] > 0:
        positionStart[2] -= (drahtD + zylinderD) / sectionsPerLoop
    else:
        positionStart[2] += (drahtD + zylinderD) / sectionsPerLoop

def subtractVector(position, origin, calculatedVector):
    calculatedVector[0] = position[0] - origin[0]
    calculatedVector[1] = position[1] - origin[1]
    calculatedVector[2] = position[2] - origin[2]

def addVector(vector1, vector2, calculatedVector):
    calculatedVector[0] = vector1[0] + vector2[0]
    calculatedVector[1] = vector1[1] + vector2[1]
    calculatedVector[2] = vector1[2] + vector2[2]

def divideVector (vector, number, calculatedVector):
    calculatedVector[0] = vector[0] / number
    calculatedVector[1] = vector[1] / number
    calculatedVector[2] = vector[2] / number

def vectorproduct (vektor1, vektor2, kreutzprodukt):
    kreutzprodukt[0] = vektor1[1]*vektor2[2] - vektor1[2]*vektor2[1]
    kreutzprodukt[1] = vektor1[2]*vektor2[0] - vektor1[0]*vektor2[2]
    kreutzprodukt[2] = vektor1[0]*vektor2[1] - vektor1[1]*vektor2[0]

def calculateVector (vector):
    return math.sqrt(vector[0]**2 + vector[1]**2 + vector[2]**2)

def calculateSum(dl, kreutzprodukt, vectorDifference, calculatedSum):
    vectorproduct(dl, vectorDifference, kreutzprodukt)
    divideVector(kreutzprodukt, calculateVector(vectorDifference)**3, calculatedSum)




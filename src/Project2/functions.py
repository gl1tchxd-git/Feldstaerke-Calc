import math

def kreutzprodukt (a, b):
    return [(a[1] * b[2] - a[2] * b[1]), (a[2] * b[0] - a[0] * b[2]), (a[0] * b[1] - a[1] * b[0])]

def calculateSum (dl, ri, r):
    x = subtractVector(r, ri)
    y = kreutzprodukt(dl, x)
    z = (abs(calculateVector(x)))**3
    return divideVector(y, z)

def calculateVector (vector):
    return math.sqrt(vector[0]**2 + vector[1]**2 + vector[2]**2)

def subtractVector (vector1, vector2):
    return [vector1[0] - vector2[0], vector1[1] - vector2[1], vector1[2] - vector2[2]]

def divideVector (vector, number):
    return [vector[0] / number, vector[1] / number, vector[2] / number]

def addVector (vector1, vector2):
    return [vector1[0] + vector2[0], vector1[1] + vector2[1], vector1[2] + vector2[2]]
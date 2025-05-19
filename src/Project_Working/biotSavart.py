import math
import array as arr

class BiotSavart:
    def __init__(self, r):
        self.amperage = 1.8 #A
        self.n = 210
        self.drahtDiameter = 0.0011 #m
        self.diameter = 0.025 + self.drahtDiameter#m
        self.zylinderL = 0.23 #m

        self.segments = 4000

        self.dl = arr.array("d", [0.0, 0.0, 0.0])
        self.ri = arr.array("d",[0.0, 0.0, -self.diameter])
        self.r = r
        self.pos = arr.array("d",[0.0, 0.0, -self.diameter])
        self.h = arr.array("d",[0.0, 0.0, 0.0])

    def crossProduct(self, vector1, vector2, vectorCalc):
        vectorCalc[0] = vector1[1] * vector2[2] - vector1[2] * vector2[1]
        vectorCalc[1] = vector1[2] * vector2[0] - vector1[0] * vector2[2]
        vectorCalc[2] = vector1[0] * vector2[1] - vector1[1] * vector2[0]


    def subVector(self, vector1, vector2, vectorCalc):
        vectorCalc[0] = vector1[0] - vector2[0]
        vectorCalc[1] = vector1[1] - vector2[1]
        vectorCalc[2] = vector1[2] - vector2[2]

    def betrag(self, vector):
        return math.sqrt(vector[0]**2 + vector[1]**2 + vector[2]**2)

    def divVector(self, vector, numb, vectorCalc):
        vectorCalc[0] = vector[0] / numb
        vectorCalc[1] = vector[1] / numb
        vectorCalc[2] = vector[2] / numb

    def changePos(self, position):
        if position[2] > 0:
            position[0] += self.diameter / self.segments
        else:
            position[0] -= self.diameter / self.segments

        position[1] += self.drahtDiameter / self.segments

        if position[0] > 0:
            position[2] -= self.diameter / self.segments
        else:
            position[2] += self.diameter / self.segments

    def calculate(self):
        for i in range(self.segments * self.n):
            rSub = arr.array("d", [0.0, 0.0, 0.0])
            sumH = arr.array("d", [0.0, 0.0, 0.0])

            self.changePos(self.pos)
            self.subVector(self.pos, self.ri, self.dl)

            self.subVector(self.r, self.ri, rSub)
            self.crossProduct(self.dl, rSub, sumH)
            self.divVector(sumH, self.betrag(rSub)**3, sumH)

            self.h[0] += sumH[0]
            self.h[1] += sumH[1]
            self.h[2] += sumH[2]

            self.changePos(self.ri)
            self.changePos(self.pos)
            self.subVector(self.pos, self.ri, self.dl)

        return self.amperage/(4.0*math.pi) * self.betrag(self.h)
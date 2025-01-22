rH = 1.5e-2 #m
amperage = 30 #A
positionStart = [1.0, 2.0, -1.0]
positionMiddle = [1.0, 2.0, -1.0]
positionEnd = [1.0, 2.0, 1.0]
sections = 2000 #n
sectionLength = (positionEnd[2] - positionStart[2]) / sections
positionH = [positionStart[0] + rH, positionStart[1], 0.0]

positionRi = [1.0, 2.0, -1.0]
origin = [0, 0, 0]
addUp = [0, 0, 0]
h = 0
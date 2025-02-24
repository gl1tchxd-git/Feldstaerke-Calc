rH = 5e-2 #m
amperage = 3.5 #A
l = 20001
positionStart = [1.0, 2.0, -l]
positionMiddle = [1.0, 2.0, -l]
positionEnd = [1.0, 2.0, l]
sections = l * 2000 #n
sectionLength = (positionEnd[2] - positionStart[2]) / sections
positionH = [positionStart[0] + rH, positionStart[1], 0.0]

positionRi = [1.0, 2.0, -l]
origin = [0, 0, 0]
addUp = [0, 0, 0]
h = 0

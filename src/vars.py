rH = 0.5 #m
amperage = 3.5 #A
vectorStart = [1.0, 2.0, -1.0]
vectorEnd = [1.0, 2.0, 1.0]
sections = 2000 #n
sectionLength = ( vectorEnd[2] - vectorStart[2] ) / sections
positionH = [vectorStart[0] + rH, vectorStart[1], 0.0]
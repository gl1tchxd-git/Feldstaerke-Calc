from biotSavart import biotSavart
import array as arr

arr1 = arr.array("d", [0, 0.115, 0])

bs1 = biotSavart(arr1)

h1 = bs1.calculate()

print(h1, "A/m")

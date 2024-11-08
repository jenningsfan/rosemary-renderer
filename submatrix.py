nums = list(range(16))
def submatrix(row, col):
    result = []
    for (i, item) in enumerate(nums):
        if ((i - col) % 4 != 0 and ((i >= (row + 1) * 4) or (i < row * 4))):
            result.append(item)
    return result

result = []

for i in range(4):
    for j in range(4):
        result.append(submatrix(i, j))

print(result)
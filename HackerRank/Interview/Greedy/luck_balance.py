#!/usr/bin/env python3

def luckBalance(k, contests):
    min_losses = sum(sorted([x[0] for x in contests if x[1] == 1], reverse=True)[k:])
    total_val = sum([x[0] for x in contests])
    return total_val - (min_losses * 2)


print(luckBalance(3, [[5,1], [2,1], [1,1], [8,1], [10,0], [5,0]]))
print(luckBalance(5, [[13, 1], [10, 1], [9, 1], [8, 1], [13, 1], [12, 1], [18, 1], [13, 1]]))
print(luckBalance(58, ([105, 0], [103, 0], [106, 1], [106, 1], [103, 0], [103, 1], [105, 1], [106, 1],
[105, 0],
[104, 0],
[103, 0],
[102, 0],
[104, 0],
[105, 0],
[104, 0],
[102, 1],
[104, 0],
[106, 1],
[104, 1],
[101, 1],
[105, 0],
[103, 0],
[104, 0],
[106, 0],
[102, 1],
[103, 0],
[102, 0],
[103, 1],
[106, 0],
[104, 1],
[101, 1],
[101, 1],
[106, 0],
[103, 1],
[103, 0],
[104, 1],
[101, 0],
[105, 1],
[105, 0],
[104, 1],
[105, 0],
[106, 0],
[104, 0],
[105, 0],
[101, 1],
[106, 1],
[105, 0],
[103, 0],
[104, 1],
[101, 1],
[106, 1],
[104, 0],
[106, 1],
[105, 0],
[103, 1],
[101, 0],
[103, 0],
[101, 0],
[105, 1],
[104, 1],
[104, 1],
[105, 1],
[105, 1],
[103, 0],
[101, 0],
[104, 1],
[106, 1],
[105, 1],
[105, 0],
[106, 1],
[104, 1],
[105, 1],
[103, 1],
[102, 1],
[106, 0],
[101, 0],
[105, 1],
[104, 1],
[103, 1],
[106, 1],
[101, 0],
[106, 1],
[103, 0],
[106, 1],
[102, 1],
[103, 0],
[101, 1],
[102, 1],
[101, 1],
[104, 0],
[106, 0],
[102, 0],
[104, 0],
[105, 0],
[105, 0],
[102, 1],
[103, 1])))
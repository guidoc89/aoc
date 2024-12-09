import heapq
import argparse


a = argparse.ArgumentParser()
# lines = open("test1.txt").readlines()
# lines = open("prod1.txt").readlines()
# lines = open("test2.txt").readlines()
lines = open("prod2.txt").readlines()
L, R = [], []

heapq.heapify(L)
heapq.heapify(R)

ans = 0
# Both part 1 and part 2
for line in lines:
    left, right = [int(x) for x in line.split()]
    heapq.heappush(L, left)
    heapq.heappush(R, right)

for n in L:
    count = R.count(n)
    ans += (count*n)

print(ans)


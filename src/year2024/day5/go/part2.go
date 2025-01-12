package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
    var ans int
    blocks := make([]string, 2)
    hm := make(map[int][]int)

    file, _ := os.ReadFile("../prod1.txt")
    blocks = strings.Split(string(file), "\n\n")

    for _, page := range strings.Split(blocks[0], "\n") {
        fields := strings.Split(page, "|")
        nums := parseInts(fields)
        hm[nums[0]] = append(hm[nums[0]], nums[1])
    }

    for _, rawUpdate := range strings.Split(blocks[1], "\n") {
        rawFields := strings.Split(rawUpdate, ",")
        update := parseInts(rawFields)

        if !isCorrect(update, hm) {
            ans += topologicalOrderingMidInt(update, hm)
        }
    }

    fmt.Printf("Ans: %d", ans)
}

func parseInts(fields []string) []int {
    nums := make([]int, len(fields))
    for i, f := range fields {
        nums[i], _ = strconv.Atoi(f)
    }

    return nums
}

func topologicalOrderingMidInt(update []int, hm map[int][]int) int {
    if len(update) <= 1 {
        return 0
    }

    inDegree := make(map[int]int, len(update))
    var q []int
    var topoOrder []int

    for _, node := range update {
        if _, exists := inDegree[node]; !exists {
            inDegree[node] = 0
        }

        if neighbours, exists := hm[node]; exists {
            for _, neighbour := range neighbours {
                if slices.Contains(update, neighbour) {
                    inDegree[neighbour]++
                }
            }
        }
    }

    // Nodes with 0 inDegree (they dont have nodes before them, so they can be used as starting points)
    for node, degree := range inDegree {
        if degree == 0 {
            q = append(q, node)
        }
    }

    for len(q) > 0 {
        node := q[0]
        topoOrder = append(topoOrder, node)
        q = q[1:]

        if neighbours, exists := hm[node]; exists {
            for _, neighbour := range neighbours {
                if _, exists := inDegree[neighbour]; exists {
                    inDegree[neighbour]--
                    if inDegree[neighbour] == 0 {
                        q = append(q, neighbour)
                    }
                }
            } 
        }
    }

    if len(topoOrder) == 0 {
        return 0
    }

    return topoOrder[len(topoOrder) /2]
}

func isCorrect(update []int, hm map[int][]int) bool {
    for i, x := range update {
        for j, y := range update {
            if j > i {
                if nums, exists := hm[x]; exists {
                    if !slices.Contains(nums, y) {
                        return false
                    }
                } else {
                    return false
                }

            }
        }
    }

    return true
}

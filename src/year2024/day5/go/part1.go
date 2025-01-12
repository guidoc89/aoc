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

        if isCorrect(update, hm) {
            ans += update[len(update) / 2]
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

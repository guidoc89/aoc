package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
    var rawPages []string
    var ans int
    var rawUpdates []string
    blocks := make([]string, 2)
    hm := make(map[int][]int)
    

    file, _ := os.ReadFile("../prod1.txt")
    blocks = strings.Split(string(file), "\n\n")

    rawPages = strings.Split(blocks[0], "\n")
    rawUpdates = strings.Split(blocks[1], "\n")

    for _, page := range rawPages {
        nums := strings.Split(page, "|")
        parsedNums := make([]int,len(nums))

        for i, n := range nums {
            parsedNums[i], _ = strconv.Atoi(n)
        }

        hm[parsedNums[0]] = append(hm[parsedNums[0]], parsedNums[1])
    }

    for _, rawUpdate := range rawUpdates {
        correct := true
        rawFields := strings.Split(rawUpdate, ",")
        update := make([]int, len(rawFields))
        for i, c := range rawFields {
            update[i], _ = strconv.Atoi(c)
        }

        for i, x := range update {
            for j, y := range update {
                if i < j {
                    // Check if X exists in the hm, and then check if y is not present in the slice
                    // NOTE: indexing the key return "exists" as boolean (not err!!)
                    if nums, exists := hm[x]; exists  {
                        if !slices.Contains(nums, y) {
                            correct = false
                            break
                        }
                    } else {
                        correct = false
                        break
                    }
                }
            }
        }
        if correct {
            ans += update[len(update) / 2]
        }
    }
    fmt.Printf("Ans: %d", ans)

}

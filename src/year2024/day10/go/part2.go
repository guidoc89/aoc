package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Position struct {
    x, y int
}

func main() {
    var ans int
    file, _ := os.ReadFile("../prod2.txt")

    lines := strings.Split(strings.TrimSpace(string(file)), "\n")
    grid := make([][]int, len(lines))

    DIRECTIONS := []Position{
        {1, 0},
        {-1, 0},
        {0, 1},
        {0, -1},
    }

    for idx, line  := range lines {
        grid[idx] = strToIntSlice(line)
    }

    nRows := len(grid)
    nCols := len(grid[0])

    for i := range nRows {
        for j := range nCols {
            if grid[i][j] == 0 {
                q := []Position{{i, j}}

                for len(q) > 0 {
                    position := q[0]
                    r, c := position.x, position. y
                    q = q[1:]

                    if grid[r][c] == 9 {
                        ans++
                    }

                    for _, delta := range DIRECTIONS {
                        newR, newC := r + delta.x, c + delta.y

                        if newR >= 0 && newR < nRows && newC >= 0 && newC < nCols && grid[newR][newC] == grid[r][c] + 1{
                            q = append(q, Position{newR, newC})
                        }
                    }
                }
            }
        }
    }

    fmt.Printf("Ans: %d", ans)
}


func strToIntSlice(s string) []int {
    strList := strings.Split(s, "")
    nums := make([]int, len(strList))

    for i, c := range strList {
        nums[i], _ = strconv.Atoi(c)
    }

    return nums
}

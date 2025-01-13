
package main

import (
	"fmt"
	"os"
	"strings"
)


type Position struct {
    x,y int
}

func main() {
    file, _ := os.ReadFile("../prod1.txt")
    cycle := 0
    grid := strings.Split(strings.TrimSpace(string(file)), "\n")
    visited := make(map[Position]struct{})
    DIRECTIONS := []Position{
        {-1,0},
        {0,1},
        {1,0},
        {0,-1},
    }

    nRows := len(grid)
    nCols := len(grid[0])

    guardPosition := getGuardPosition(grid)
    visited[guardPosition]= struct{}{}

    for {
        currDirection := DIRECTIONS[cycle]
        newPosition := Position{guardPosition.x + currDirection.x, guardPosition.y + currDirection.y}

        if  newPosition.x >= 0  && newPosition.x < nRows &&  newPosition.y >= 0 && newPosition.y < nCols && grid[newPosition.x][newPosition.y] != '#' {
            guardPosition = newPosition
            visited[guardPosition] = struct{}{}
        } else {
            cycle = (cycle + 1) % len(DIRECTIONS)
        }

        if (newPosition.x < 0) || (newPosition.x >= nRows) || (newPosition.y < 0) || (newPosition.y >= nCols) {
            break
        }
    }


    fmt.Printf("Ans: %d", len(visited))
}

func getGuardPosition(grid []string) Position {
    nRows := len(grid)
    nCols := len(grid[0])

    for x := 0; x < nRows; x++ {
        for y:= 0; y < nCols; y++ {
            if string(grid[x][y]) == "^" {
                return Position{x,y}
            }
        }
    }

    return Position{-1,-1}
}

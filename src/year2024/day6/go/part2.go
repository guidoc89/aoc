
package main

import (
	"fmt"
	"os"
	"strings"
)


type Position struct {
    x, y int
}


type PositionDirection struct {
    pos, dir Position
}

func main() {
    file, _ := os.ReadFile("../prod2.txt")
    var ans int
    lines := strings.Split(strings.TrimSpace(string(file)), "\n")
    grid := make([][]string, len(lines))
    for i, line := range lines {
        grid[i] = strings.Split(line, "")
    }

    DIRECTIONS := []Position{
        {-1,0},
        {0,1},
        {1,0},
        {0,-1},
    }

    nRows := len(grid)
    nCols := len(grid[0])

    originalGuardPositionDirection := getGuardPositionDirection(grid)

    for i := range nRows {
        for j := range nCols {
            if grid[i][j] != "." || (i == originalGuardPositionDirection.pos.x && j == originalGuardPositionDirection.pos.y) {
                continue
            } else {
                cycle := 0
                visited := make(map[PositionDirection]struct{})
                guardPositionDirection := originalGuardPositionDirection
                grid[i][j] = "#"

                for {
                    currDirection := DIRECTIONS[cycle]
                    currPosition := guardPositionDirection.pos
                    guardPositionDirection = PositionDirection{
                        pos: currPosition,
                        dir: currDirection,
                    }

                    if _, seen := visited[guardPositionDirection]; seen {
                        ans++
                        break
                    }

                    visited[guardPositionDirection] = struct{}{}

                    newPosition := Position{
                        currPosition.x + currDirection.x,
                        currPosition.y + currDirection.y,
                    }


                    if  newPosition.x >= 0  && newPosition.x < nRows &&  newPosition.y >= 0 && newPosition.y < nCols && grid[newPosition.x][newPosition.y] != "#" {
                        guardPositionDirection = PositionDirection{
                            pos: newPosition,
                            dir: currDirection,
                        }
                    } else {
                        cycle = (cycle + 1) % len(DIRECTIONS)
                    }

                    if (newPosition.x < 0) || (newPosition.x >= nRows) || (newPosition.y < 0) || (newPosition.y >= nCols) {
                       break
                    }
                }

                grid[i][j] = "."
            }
        }
    }



    fmt.Printf("Ans: %d", ans)
}

func getGuardPositionDirection(grid [][]string) PositionDirection {
    nRows := len(grid)
    nCols := len(grid[0])

    for x := 0; x < nRows; x++ {
        for y:= 0; y < nCols; y++ {
            if grid[x][y] == "^" {
                return PositionDirection{
                    Position{x,y},
                    Position{-1,0}, // start dir: UP
                }
            }
        }
    }

    return PositionDirection{
        Position{-1,-1},
        Position{-1,0}, // start dir: UP
    }
}


package main


import (
    "fmt"
    "bufio"
    "os"
)

func main() {
    file, _ := os.Open("../prod2.txt")
    var grid []string
    var ans int

    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        grid = append(grid, scanner.Text())
    }

    var nRows = len(grid)
    var nCols = len(grid[0])


    for i := range nRows {
        for j := range nCols {

            // M   S
            //   A
            // M   S
            if i+2 < nRows && j+2 < nCols && rune(grid[i][j]) == 'M' && rune(grid[i+1][j+1]) == 'A' && rune(grid[i+2][j+2]) == 'S' && rune(grid[i][j+2]) == 'S' && rune(grid[i+2][j]) == 'M' {
                ans++
            }

            // S   S
            //   A
            // M   M
            if i+2 < nRows && j+2 < nCols && rune(grid[i][j]) == 'S' && rune(grid[i+1][j+1]) == 'A' && rune(grid[i+2][j+2]) == 'M' && rune(grid[i][j+2]) == 'S' && rune(grid[i+2][j]) == 'M' {
                ans++
            }

            // S   M
            //   A
            // S   M
            if i+2 < nRows && j+2 < nCols && rune(grid[i][j]) == 'S' && rune(grid[i+1][j+1]) == 'A' && rune(grid[i+2][j+2]) == 'M' && rune(grid[i][j+2]) == 'M' && rune(grid[i+2][j]) == 'S' {
                ans++
            }

            // M   M
            //   A
            // S   S
            if i+2 < nRows && j+2 < nCols && rune(grid[i][j]) == 'M' && rune(grid[i+1][j+1]) == 'A' && rune(grid[i+2][j+2]) == 'S' && rune(grid[i][j+2]) == 'M' && rune(grid[i+2][j]) == 'S' {
                ans++
            }
        }
    }

    fmt.Println(ans)
}

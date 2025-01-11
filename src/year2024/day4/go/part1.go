
package main


import (
    "fmt"
    "bufio"
    "os"
)

func main() {
    file, _ := os.Open("../prod1.txt")
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
            // XMAS
            if j+3 < nCols && rune(grid[i][j]) == 'X' && rune(grid[i][j+1]) == 'M' && rune(grid[i][j+2]) == 'A' && rune(grid[i][j+3]) == 'S' {
                ans++
            }
            // SAMX
            if j+3 < nCols && rune(grid[i][j]) == 'S' && rune(grid[i][j+1]) == 'A' && rune(grid[i][j+2]) == 'M' && rune(grid[i][j+3]) == 'X' {
                ans++
            }
            // XMAS (diagonally from the top-left to the bottom-right)
            if i+3 < nRows && j+3 < nCols && i < nRows && rune(grid[i][j]) == 'X' && rune(grid[i+1][j+1]) == 'M' && rune(grid[i+2][j+2]) == 'A' && rune(grid[i+3][j+3]) == 'S' {
                ans++
            }
            // SAMX (diagonally from the bottom-right to the top-left)
            if i+3 < nRows && j+3 < nCols && rune(grid[i][j]) == 'S' && rune(grid[i+1][j+1]) == 'A' && rune(grid[i+2][j+2]) == 'M' && rune(grid[i+3][j+3]) == 'X' {
                ans++
            }
            // XMAS (top to bottom)
            if i+3 < nRows && rune(grid[i][j]) == 'X' && rune(grid[i+1][j]) == 'M' && rune(grid[i+2][j]) == 'A' && rune(grid[i+3][j]) == 'S' {
                ans++
            }
            // SAMS (bottom to top)
            if i+3 < nRows && rune(grid[i][j]) == 'S' && rune(grid[i+1][j]) == 'A' && rune(grid[i+2][j]) == 'M' && rune(grid[i+3][j]) == 'X' {
                ans++
            }
            // XMAS (diagonally from the bottom-left to the top-right)
            if i-3 >= 0 && j+3 < nCols && i < nRows && rune(grid[i][j]) == 'X' && rune(grid[i-1][j+1]) == 'M' && rune(grid[i-2][j+2]) == 'A' && rune(grid[i-3][j+3]) == 'S' {
                ans++
            }
            // SAMX (diagonally from the top-right to the top-right)
            if i-3 >= 0 && j+3 < nCols && i < nRows && rune(grid[i][j]) == 'S' && rune(grid[i-1][j+1]) == 'A' && rune(grid[i-2][j+2]) == 'M' && rune(grid[i-3][j+3]) == 'X' {
                ans++
            }
        }
    }

    fmt.Println(ans)
}

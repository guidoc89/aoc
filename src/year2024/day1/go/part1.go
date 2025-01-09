
package main

import (
    "fmt"
    "strings"
    "sort"
    "strconv"
    "os"
    "bufio"
)

func main() {
    file, _ := os.Open("../prod1.txt")
    defer file.Close()

    var lines []string
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }

    var l, r []int

    for _, line := range lines {
        nums := strings.Fields(line)
        if value, err := strconv.Atoi(nums[0]); err == nil {
            l = append(l, value)

        }

        if value, err := strconv.Atoi(nums[1]); err == nil {
            r = append(r, value)
        }
    }

    // Sorts in place
    sort.Ints(l)
    sort.Ints(r)

    var ans int = 0

    for i := 0; i < len(l); i++ {
        diff := l[i] - r[i]
        ans += max(diff, -diff)

    }

    fmt.Println("Ans: ", ans)
}



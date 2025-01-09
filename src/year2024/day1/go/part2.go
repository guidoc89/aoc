
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
    file, _ := os.Open("../prod2.txt")
    defer file.Close()

    var lines []string
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }

    var l, r []int

    // NOTE: see https://go.dev/blog/maps as to why need to use make() to initialize it
    counts := make(map[int]int)

    for _, line := range lines {
        nums := strings.Fields(line)
        if value, err := strconv.Atoi(nums[0]); err == nil {
            l = append(l, value)

        }

        if value, err := strconv.Atoi(nums[1]); err == nil {
            r = append(r, value)
            // This works! since an uninitialized int is 0
            counts[value]++
        }
    }

    sort.Ints(l)
    sort.Ints(r)

    var ans int = 0
    for _, left_value := range l {
        ans += (counts[left_value] * left_value)
    }

    fmt.Println("Ans: ", ans)
}




package main

import (
    "bufio"
    "fmt"
    "strings"
    "strconv"
    "os"
)

func main() {
    file, _ := os.Open("../prod2.txt")
    defer file.Close()

    var lines []string
    ans := 0
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }

    enabled := true
    for _, line := range lines {
        for i := range line {

            if strings.HasPrefix(line[i:], "do()") {
                enabled = true
                continue
            }
            if strings.HasPrefix(line[i:], "don't()") {
                enabled = false
                continue
            }

            if strings.HasPrefix(line[i:], "mul(") {
                end := i

                // NOTE: this would be a while loop in Go lol...
                // It works like rust, single quotes == char (byte/rune comparison works in this case)
                for line[end] != ')' {
                    end++
                }

                new_slice := strings.Replace(line[i:end+1], "mul(", "", -1)
                new_slice = strings.Replace(new_slice, ")", "", -1)
                splits := strings.Split(new_slice, ",")

                if enabled {
                    if len(splits) == 2 {
                        val1, _ := strconv.Atoi(splits[0])
                        val2, _ := strconv.Atoi(splits[1])
                        ans += (val1 * val2)
                    }
                }
            }
        }
    }
    fmt.Println("Ans: ", ans)
}


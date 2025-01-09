
package main


import (
    "fmt"
    "bufio"
    "strings"
    "strconv"
    "os"
)


func main() {
    file, _ := os.Open("../prod2.txt")
    defer file.Close()
    var ans int


    var lines []string
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }

    for _, line := range lines {
        nums := parse_line(line)
        var safe = false
        if is_safe(nums) {
            safe = true
        } else {
                for i := 0; i < len(nums); i++ {
                    new_nums := make([]int, 0, len(nums)-1)
                    new_nums = append(new_nums, nums[:i]...)
                    new_nums = append(new_nums, nums[i+1:]...)
                    if is_safe(new_nums) {
                        safe = true
                        break
                    }
                }
        }

        if safe {
            ans++
        }
    }

    fmt.Println("Ans: ", ans)
}


func parse_line(line string) []int {
    nums := strings.Fields(line)
    parsed_nums := make([]int, len(nums))

    for i, c := range nums {
        if val, err := strconv.Atoi(c); err == nil {
            parsed_nums[i] = val
        }
    }
    return parsed_nums
}

func is_safe(nums []int) bool {
    allPositive := true
    allNegative := true
    maxThree := true
    minOne := true

    for i := 1; i < len(nums); i++ {
        diff := nums[i-1] - nums[i]

        if diff >= 0 {
            allNegative = false
        }

        if diff <= 0 {
            allPositive = false
        } 

        abs_diff := max(diff, -diff)
        if abs_diff < 1 {
            minOne = false
        }

        if abs_diff > 3 {
            maxThree = false
        }
    }
    return (allPositive || allNegative) && maxThree && minOne
}


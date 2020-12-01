package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	lvl1 := map[int]bool{}
	lvl2 := map[int][2]int{}

	target := 2020
	for scanner.Scan() {
		num, err := strconv.Atoi(scanner.Text())
		if err != nil {
			continue
		}

		missing := target - num
		if found, ok := lvl2[missing]; ok {
			fmt.Println(num * found[0] * found[1])
			return
		} else if _, ok := lvl1[num]; !ok {
			lvl1[num] = true
			for val := range lvl1 {
				lvl2[val+num] = [2]int{val, num}
			}
		}
	}
}

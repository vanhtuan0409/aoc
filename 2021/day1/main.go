package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	data, err := ioutil.ReadAll(os.Stdin)
	if err != nil {
		panic(err)
	}

	lines := strings.Split(string(data), "\n")
	slice := []int{}
	for _, l := range lines {
		if strings.TrimSpace(l) == "" {
			continue
		}

		num, err := strconv.Atoi(l)
		if err != nil {
			panic(err)
		}
		slice = append(slice, num)
	}

	lastSum := math.MaxInt
	count := 0
	for idx := range slice {
		if idx < 1 || idx >= len(slice)-1 {
			continue
		}

		sum := slice[idx-1] + slice[idx] + slice[idx+1]
		if sum > lastSum {
			count++
		}
		lastSum = sum
	}

	fmt.Println("total", count)
}

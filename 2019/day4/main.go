package main

import "fmt"

func toDigits(num int) []int {
	ret := []int{}
	for num > 0 {
		d := num % 10
		ret = append([]int{d}, ret...)
		num = num / 10
	}
	return ret
}

func isMatch(pass int) bool {
	digits := toDigits(pass)
	dynamicIncreased := make([]int, len(digits))
	dynamicDoubled := make([]int, len(digits))
	magicArray := []int{}

	for index, d := range digits {
		if index == 0 {
			dynamicIncreased[index] = 1
			dynamicDoubled[index] = 1
			continue
		}

		if d >= digits[index-1] {
			dynamicIncreased[index] = 1
		}
		if d == digits[index-1] {
			dynamicDoubled[index] = dynamicDoubled[index-1] + 1
		} else {
			magicArray = append(magicArray, dynamicDoubled[index-1])
			dynamicDoubled[index] = 1
		}
	}
	magicArray = append(magicArray, dynamicDoubled[len(digits)-1])

	for _, i := range dynamicIncreased {
		if i == 0 {
			return false
		}
	}

	for _, i := range magicArray {
		if i == 2 {
			return true
		}
	}

	return false
}

func main() {
	fmt.Println(isMatch(111122))

	count := 0
	for i := 158126; i <= 624574; i++ {
		if isMatch(i) {
			count++
		}
	}
	fmt.Println(count)
}

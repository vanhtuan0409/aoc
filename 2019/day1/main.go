package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func calculateFuel(mass int) int {
	return mass/3 - 2
}

func recursiveCalculateFuel(mass int) int {
	requiredFuel := calculateFuel(mass)
	if requiredFuel < 0 {
		return 0
	}
	return requiredFuel + recursiveCalculateFuel(requiredFuel)
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	sum := 0
	for scanner.Scan() {
		mass, err := strconv.Atoi(scanner.Text())
		if err != nil {
			continue
		}
		sum += recursiveCalculateFuel(mass)
	}
	fmt.Println(sum)
}

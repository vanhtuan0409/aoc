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

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	sum := 0
	for scanner.Scan() {
		mass, err := strconv.Atoi(scanner.Text())
		if err != nil {
			continue
		}
		sum += calculateFuel(mass)
	}
	fmt.Println(sum)
}

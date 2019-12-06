package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func countOrbits(n string, m map[string][]string) int {
	if len(m[n]) == 0 {
		return 0
	}
	count := 0
	for _, p := range m[n] {
		count += 1 + countOrbits(p, m)
	}
	return count
}

func parseCommand(input string) (string, string) {
	parts := strings.Split(input, ")")
	return parts[0], parts[1]
}

func main() {
	r := bufio.NewScanner(os.Stdin)
	m := make(map[string][]string)
	m["COM"] = []string{}

	for r.Scan() {
		input := r.Text()
		root, orbiter := parseCommand(input)
		_, ok := m[orbiter]
		if ok {
			m[orbiter] = append(m[orbiter], root)
		} else {
			m[orbiter] = []string{root}
		}
	}

	count := 0
	for n := range m {
		count += countOrbits(n, m)
	}
	fmt.Println(count)
}

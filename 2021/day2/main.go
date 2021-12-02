package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type position struct {
	horizon int
	depth   int
	aim     int
}

func main() {
	s := bufio.NewScanner(os.Stdin)

	curr := position{
		horizon: 0,
		depth:   0,
		aim:     0,
	}
	for s.Scan() {
		line := s.Text()
		direction, val, err := parse(line)
		if err != nil {
			panic(err)
		}

		switch direction {
		case "forward":
			curr = position{
				horizon: curr.horizon + val,
				depth:   curr.depth + curr.aim*val,
				aim:     curr.aim,
			}
		case "down":
			curr = position{
				horizon: curr.horizon,
				depth:   curr.depth,
				aim:     curr.aim + val,
			}
		case "up":
			curr = position{
				horizon: curr.horizon,
				depth:   curr.depth,
				aim:     curr.aim - val,
			}
		default:
			panic("invalid direction " + direction)
		}
	}

	fmt.Println("res:", curr.depth*curr.horizon)
}

func parse(in string) (string, int, error) {
	parts := strings.Split(in, " ")
	if len(parts) != 2 {
		return "", 0, errors.New("invalid input")
	}
	val, err := strconv.Atoi(parts[1])
	if err != nil {
		return "", 0, err
	}

	return parts[0], val, nil
}

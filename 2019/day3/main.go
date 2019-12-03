package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func max(i1, i2 int) int {
	if i1 > i2 {
		return i1
	}
	return i2
}

func min(i1, i2 int) int {
	if i1 > i2 {
		return i2
	}
	return i1
}

func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

type point struct {
	x int
	y int
}

func newPoint(x, y int) *point {
	return &point{x, y}
}

func (p *point) same(o *point) bool {
	return p.x == o.x && p.y == o.y
}

func (p *point) ToString() string {
	return fmt.Sprintf("(%d, %d)", p.x, p.y)
}

type segment struct {
	p1 *point
	p2 *point
}

func newSegment(p1, p2 *point) *segment {
	return &segment{p1, p2}
}

func (s *segment) getOrientation() int {
	if s.p1.x == s.p2.x {
		return 1
	}
	return -1
}

func (s *segment) len() int {
	if s.getOrientation() == 1 {
		return abs(s.p1.y - s.p2.y)
	}
	return abs(s.p1.x - s.p2.x)
}

func (s *segment) ToString() string {
	return fmt.Sprintf("%s, %s", s.p1.ToString(), s.p2.ToString())
}

func intersect(horizontal, vertical *segment) *point {
	if horizontal.getOrientation()*vertical.getOrientation() > 0 {
		return nil
	}
	if horizontal.getOrientation() != 1 {
		return intersect(vertical, horizontal)
	}
	if min(horizontal.p1.y, horizontal.p2.y) > vertical.p1.y || max(horizontal.p1.y, horizontal.p2.y) < vertical.p1.y {
		return nil
	}
	if min(vertical.p1.x, vertical.p2.x) > horizontal.p1.x || max(vertical.p1.x, vertical.p2.x) < horizontal.p1.x {
		return nil
	}
	return newPoint(horizontal.p1.x, vertical.p1.y)
}

func decomposeDirection(direction string) (string, int) {
	orientation := strings.ToUpper(string(direction[0]))
	value, _ := strconv.Atoi(direction[1:])
	return orientation, value
}

func nextPoint(p *point, direction string) *point {
	orientation, value := decomposeDirection(direction)
	if orientation == "R" {
		return newPoint(p.x+value, p.y)
	} else if orientation == "L" {
		return newPoint(p.x-value, p.y)
	} else if orientation == "U" {
		return newPoint(p.x, p.y+value)
	} else {
		return newPoint(p.x, p.y-value)
	}
}

func tracing(p *point, ds []string) []*segment {
	ret := []*segment{}
	for _, direction := range ds {
		next := nextPoint(p, direction)
		ret = append(ret, newSegment(p, next))
		p = next
	}
	return ret
}

func manhattanDist(p1, p2 *point) int {
	return abs(p1.x-p2.x) + abs(p1.y-p2.y)
}

func part1() {
	r := bufio.NewReader(os.Stdin)
	l1, _ := r.ReadString('\n')
	l2, _ := r.ReadString('\n')
	ray1 := strings.Split(strings.TrimSpace(l1), ",")
	ray2 := strings.Split(strings.TrimSpace(l2), ",")

	O := newPoint(0, 0)
	segs1 := tracing(O, ray1)
	segs2 := tracing(O, ray2)

	points := []*point{}
	for _, s1 := range segs1 {
		for _, s2 := range segs2 {
			i := intersect(s1, s2)
			if i != nil && !i.same(O) {
				points = append(points, i)
			}
		}
	}

	minDist := math.MaxInt64
	for _, p := range points {
		minDist = min(minDist, manhattanDist(O, p))
	}

	fmt.Println(minDist)
}

func isPassing(s *segment, p *point) int {
	isHorizontalPassing := s.getOrientation() == -1 && p.y == s.p1.y && p.x >= min(s.p1.x, s.p2.x) && p.x <= max(s.p1.x, s.p2.x)
	isVerticalPassing := s.getOrientation() == 1 && p.x == s.p1.x && p.y >= min(s.p1.y, s.p2.y) && p.y <= max(s.p1.y, s.p2.y)
	if isHorizontalPassing {
		return abs(s.p1.x - p.x)
	}
	if isVerticalPassing {
		return abs(s.p1.y - p.y)
	}
	return -1
}

func countStep(p *point, segs []*segment) int {
	count := 0
	for _, s := range segs {
		passingCount := isPassing(s, p)
		if passingCount == -1 {
			count += s.len()
		} else {
			count += passingCount
			return count
		}
	}
	return -1
}

func main() {
	r := bufio.NewReader(os.Stdin)
	l1, _ := r.ReadString('\n')
	l2, _ := r.ReadString('\n')
	ray1 := strings.Split(strings.TrimSpace(l1), ",")
	ray2 := strings.Split(strings.TrimSpace(l2), ",")

	O := newPoint(0, 0)
	segs1 := tracing(O, ray1)
	segs2 := tracing(O, ray2)

	points := []*point{}
	for _, s1 := range segs1 {
		for _, s2 := range segs2 {
			i := intersect(s1, s2)
			if i != nil && !i.same(O) {
				points = append(points, i)
			}
		}
	}

	countMin := math.MaxInt64
	for _, p := range points {
		countMin = min(countMin, countStep(p, segs1)+countStep(p, segs2))
	}
	fmt.Println(countMin)
}

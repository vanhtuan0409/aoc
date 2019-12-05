package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type program struct {
	originalIntCode []int
	intcode         []int
	offset          int
}

func newProgram(intcode []int) *program {
	ret := new(program)

	ret.originalIntCode = make([]int, len(intcode))
	ret.intcode = make([]int, len(intcode))
	copy(ret.originalIntCode, intcode)
	copy(ret.intcode, intcode)
	ret.offset = 0
	return ret
}

func (p *program) reset() {
	copy(p.intcode, p.originalIntCode)
	p.offset = 0
}

func (p *program) decodeCommand() (int, []int) {
	currentValue := p.getOffset(p.offset)
	command := fmt.Sprintf("%05d", currentValue)
	opcode, _ := strconv.Atoi(command[len(command)-2:])
	modes := make([]int, 3)
	for i := 0; i < 3; i++ {
		m, _ := strconv.Atoi(string(command[i]))
		modes[2-1] = m
	}
	return opcode, modes
}

func (p *program) getOffset(offset int) int {
	if offset >= len(p.intcode) {
		return -1
	}
	return p.intcode[offset]
}

func (p *program) getPointer(offset int) int {
	pointer := p.intcode[offset]
	return p.getOffset(pointer)
}

func (p *program) getValue(mode, offset int) int {
	if mode == 0 {
		return p.getPointer(offset)
	}
	return p.getOffset(offset)
}

func (p *program) set(offset, value int) {
	p.intcode[offset] = value
}

func (p *program) setPointer(offset, value int) {
	pointer := p.intcode[offset]
	p.set(pointer, value)
}

func (p *program) setOffset(offset int) {
	p.offset = offset
}

func (p *program) handleAdd(modes []int) error {
	val1 := p.getValue(modes[1], p.offset+1)
	val2 := p.getValue(modes[2], p.offset+2)
	p.setPointer(p.offset+3, val1+val2)
	p.setOffset(p.offset + 4)
	return nil
}

func (p *program) handleMultiply(modes []int) error {
	val1 := p.getValue(modes[1], p.offset+1)
	val2 := p.getValue(modes[2], p.offset+2)
	p.setPointer(p.offset+3, val1*val2)
	p.setOffset(p.offset + 4)
	return nil
}

func (p *program) execute() error {
	for {
		code, modes := p.decodeCommand()
		if code == 99 {
			break
		} else if code == 1 {
			p.handleAdd(modes)
		} else if code == 2 {
			p.handleMultiply(modes)
		} else {
			return errors.New("Unknown opcode")
		}
	}
	return nil
}

func (p *program) getResult() int {
	return p.getOffset(0)
}

func convert(arr []string) []int {
	ret := []int{}
	for _, i := range arr {
		converted, _ := strconv.Atoi(i)
		ret = append(ret, converted)
	}
	return ret
}

func setInput(p *program, val1, val2 int) {
	p.intcode[1] = val1
	p.intcode[2] = val2
}

func loadProgram() *program {
	r := bufio.NewReader(os.Stdin)
	rawData, _, _ := r.ReadLine()
	intcode := convert(strings.Split(string(rawData), ","))
	p := newProgram(intcode)
	return p
}

func part1() {
	p := loadProgram()
	setInput(p, 12, 2)
	if err := p.execute(); err != nil {
		log.Fatal(err)
	} else {
		fmt.Println(p.getResult())
	}
}

func main() {
	p := loadProgram()
	for i := 1; i < 99; i++ {
		for j := 1; j < 99; j++ {
			setInput(p, i, j)
			p.execute()
			if p.getResult() == 19690720 {
				fmt.Println(100*i + j)
			}
			p.reset()
		}
	}
}

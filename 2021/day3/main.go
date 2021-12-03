package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	s := bufio.NewScanner(os.Stdin)

	in := [][]byte{}
	for s.Scan() {
		line := s.Text()
		bs := convertByteString(line)
		in = append(in, bs)
	}

	oxygenRate := lookup(in, true, 0)
	co2Rate := lookup(in, false, 0)

	fmt.Println("oxy", oxygenRate)
	fmt.Println("co2", co2Rate)
	fmt.Println("result", convertByteInt(oxygenRate)*convertByteInt(co2Rate))
}

func lookup(in [][]byte, selectMostCommon bool, idx int) []byte {
	if len(in) == 1 {
		return in[0]
	}

	bs := extractSlice(in, idx)
	mostCommon, leastCommon := commonBit(bs)
	var filterBit byte
	if selectMostCommon {
		filterBit = mostCommon
	} else {
		filterBit = leastCommon
	}

	filtered := filterSlice(in, filterBit, idx)

	return lookup(filtered, selectMostCommon, idx+1)
}

func extractSlice(in [][]byte, idx int) []byte {
	ret := make([]byte, len(in))
	for i, bs := range in {
		ret[i] = bs[idx]
	}
	return ret
}

func filterSlice(in [][]byte, bit byte, idx int) [][]byte {
	ret := [][]byte{}
	for _, bs := range in {
		if bs[idx] == bit {
			ret = append(ret, bs)
		}
	}
	return ret
}

func commonBit(in []byte) (byte, byte) {
	oneC := 0
	zeroC := 0
	for _, b := range in {
		if b == 0 {
			zeroC++
		} else {
			oneC++
		}
	}

	if oneC >= zeroC {
		return 1, 0
	}
	return 0, 1
}

func convertByteString(in string) []byte {
	runes := []rune(in)
	ret := make([]byte, len(runes))
	for idx, r := range runes {
		if r == '0' {
			ret[idx] = 0
		} else if r == '1' {
			ret[idx] = 1
		} else {
			panic("invalid binary num " + string(r))
		}
	}
	return ret
}

func convertByteInt(in []byte) int {
	ret := 0
	for idx, b := range in {
		level := len(in) - 1 - idx
		ret += int(b) * pow2(level)
	}
	return ret
}

func pow2(pow int) int {
	ret := 1
	for i := 0; i < pow; i++ {
		ret *= 2
	}
	return ret
}

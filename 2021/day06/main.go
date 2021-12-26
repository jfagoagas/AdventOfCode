package main

import (
	"flag"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var (
	inputPath = flag.String("f", "input", "Puzzle input path")
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	flag.Parse()
	input := readLines(*inputPath)
	fmt.Printf("Part 1: %d\n", countLanternfish(input, 80))
	fmt.Printf("Part 2: %d\n", countLanternfish(input, 256))
}

func countLanternfish(input []int, maxDays int) int {
	var fishes [9]int
	for _, value := range input {
		fishes[value]++
	}

	for day := 0; day < maxDays; day++ {
		zeros := fishes[0]
		fishes[0] = 0

		for i := 1; i <= 8; i++ {
			count := fishes[i]
			fishes[i] -= count
			fishes[i-1] += count
		}

		fishes[8] += zeros
		fishes[6] += zeros

	}

	count := 0
	for i := range fishes {
		count += fishes[i]
	}
	return count
}

func readLines(path string) []int {
	file, err := os.ReadFile(path)
	check(err)

	var input []int
	for _, line := range strings.Split(strings.TrimSpace(string(file)), ",") {
		item, err := strconv.Atoi(line)
		check(err)
		input = append(input, item)
	}
	return input
}

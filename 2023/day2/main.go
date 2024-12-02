package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func part1(input string) int {
	lines := strings.Split(input, "\n")
	lines = lines[:len(lines)-1]

	cubes := map[string]int{
		"red":   12,
		"green": 13,
		"blue":  14,
	}

	sum := 0
	for i, line := range lines {
		s := strings.SplitN(line, ": ", 2)
		sets := strings.Split(s[1], "; ")

		possible := true
		for _, set := range sets {
			showed := strings.Split(set, ", ")
			for _, c := range showed {
				cc := strings.SplitN(c, " ", 2)
				count, _ := strconv.Atoi(cc[0])
				color := cc[1]
				if cubes[color] < count {
					possible = false
					break
				}
			}
		}
		if possible {
			sum += i + 1
		}
	}
	return sum
}

func part2(input string) int {
	lines := strings.Split(input, "\n")
	lines = lines[:len(lines)-1]

	total := 0
	for _, line := range lines {
		s := strings.SplitN(line, ": ", 2)
		sets := strings.Split(s[1], "; ")

		red, green, blue := 0, 0, 0
		for _, set := range sets {
			cubes := strings.Split(set, ", ")
			for _, c := range cubes {
				cc := strings.SplitN(c, " ", 2)
				n, _ := strconv.Atoi(cc[0])
				color := cc[1]

				switch color {
				case "red":
					if n > red {
						red = n
					}
				case "green":
					if n > green {
						green = n
					}
				case "blue":
					if n > blue {
						blue = n
					}
				}
			}
		}

		total += red * green * blue
	}

	return total
}

func main() {
	bytes, err := os.ReadFile("inputs/2_input.txt")
	input := string(bytes)

	if err != nil {
		log.Fatal(err)
		return
	}

	res := part1(input)
	fmt.Println(res)
	fmt.Println(part2(input))
}

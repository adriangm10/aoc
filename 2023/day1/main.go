package main

import (
	"bytes"
	"errors"
	"fmt"
	"log"
	"os"
)

var dict = map[string]int{
	"one":   1,
	"two":   2,
	"three": 3,
	"four":  4,
	"five":  5,
	"six":   6,
	"seven": 7,
	"eight": 8,
	"nine":  9,
}

func is_digit(b byte) bool {
	return b >= '0' && b <= '9'
}

func digit_to_int(b byte) int {
	return int(b - '0')
}

func find_first_digit(line []byte) (int, error) {
	for i, char := range line {
		if is_digit(char) {
			return digit_to_int(char), nil
		} else {
			if i+3 < len(line) && dict[string(line[i:i+3])] != 0 {
				return dict[string(line[i:i+3])], nil
			} else if i+4 < len(line) && dict[string(line[i:i+4])] != 0 {
				return dict[string(line[i:i+4])], nil
			} else if i+5 < len(line) && dict[string(line[i:i+5])] != 0 {
				return dict[string(line[i:i+5])], nil
			}
		}
	}
	return -1, errors.New("There is no digit in the line")
}

func find_last_digit(line []byte) (int, error) {
	for i := len(line) - 1; i >= 0; i-- {
		if is_digit(line[i]) {
			return digit_to_int(line[i]), nil
		} else {
			if i-3 >= 0 && dict[string(line[i-2:i+1])] != 0 {
				return dict[string(line[i-2:i+1])], nil
			} else if i-4 >= 0 && dict[string(line[i-3:i+1])] != 0 {
				return dict[string(line[i-3:i+1])], nil
			} else if i-5 >= 0 && dict[string(line[i-4:i+1])] != 0 {
				return dict[string(line[i-4:i+1])], nil
			}
		}
	}
	return -1, errors.New("There is no digit in the line")
}

func main() {
	input, err := os.ReadFile("inputs/1_input.txt")
	if err != nil {
		log.Fatal(err)
		return
	}

	lines := bytes.Split(input, []byte("\n"))
	lines = lines[:len(lines)-1]

	var numbers []int

	for _, line := range lines {
		first, _ := find_first_digit(line)
		last, _ := find_last_digit(line)
		numbers = append(numbers, first*10+last)
	}

	sum := 0
	for _, n := range numbers {
		sum += n
	}

	fmt.Println(sum)
}

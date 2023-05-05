package main

import (
	"fmt"

	"github.com/anagilda/advent-of-code/2022/utils"
)

// main executes the solution to the Advent of Code day 06.
func main() {

	datatstream := utils.ReadInput()[0]

	partOne(datatstream)
	partTwo(datatstream)
}

// partOne solves the first part of the Advent of Code day 06.
func partOne(datatstream string) {
	fmt.Println("Solution for part 1:")
	n := 4

	for index, _ := range datatstream {
		if index < n {
			continue
		}

		possibleStartSequence := datatstream[index-n : index]

		var presentValues [256]bool
		uniqueValues := 0
		for _, character := range possibleStartSequence {
			if presentValues[character] {
				continue
			}

			presentValues[character] = true
			uniqueValues += 1
		}

		if uniqueValues == n {
			fmt.Println(possibleStartSequence)
			fmt.Printf("Index: %v\n", index)
			break

		}

	}
}

// partTwo solves the second part of the Advent of Code day 06.
// How many characters need to be processed before the first start-of-message marker is detected?
func partTwo(datatstream string) {
	fmt.Println("Solution for part 2:")
	n := 14

	for index, _ := range datatstream {
		if index < n {
			continue
		}

		possibleStartSequence := datatstream[index-n : index]

		var presentValues [256]bool
		uniqueValues := 0
		for _, character := range possibleStartSequence {
			if presentValues[character] {
				continue
			}

			presentValues[character] = true
			uniqueValues += 1
		}

		if uniqueValues == n {
			fmt.Println(possibleStartSequence)
			fmt.Printf("Index: %v\n", index)
			break

		}

	}
}

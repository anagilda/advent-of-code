package main

import (
	"fmt"
	"strings"

	"github.com/anagilda/advent-of-code/2022/utils"
)

// main executes the solution to the Advent of Code day 03.
func main() {
	rucksackItems := utils.ReadInput()

	partOne(rucksackItems)
	partTwo(rucksackItems)
}

// partOne solves the first part of the Advent of Code day 03.
// Find the item type that appears in both compartments of each rucksack.
// What is the sum of the priorities of those item types?
func partOne(rucksackItems []string) {
	fmt.Println("Solution for part 1:")

	var itemsRepeatedInAllRucksacks []string

	for _, rucksack := range rucksackItems {
		numberOfItemsInRucksack := len(rucksack)
		numberOfItemsInCompartment := numberOfItemsInRucksack / 2

		firstCompartment, secondCompartment := rucksack[:numberOfItemsInCompartment], rucksack[numberOfItemsInCompartment:]

		var itemsRepeatedInBothCompartments []string
		for _, item := range firstCompartment {
			item := string(item)
			if strings.Contains(secondCompartment, item) {
				if !contains(itemsRepeatedInBothCompartments, item) {
					itemsRepeatedInBothCompartments = append(itemsRepeatedInBothCompartments, item)
					itemsRepeatedInAllRucksacks = append(itemsRepeatedInAllRucksacks, item)
				}
			}
		}

		if len(itemsRepeatedInBothCompartments) == 0 {
			itemsRepeatedInAllRucksacks = append(itemsRepeatedInAllRucksacks, "-")
		}

	}
	// fmt.Println(itemsRepeatedInAllRucksacks)

	var possibleItems string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	var prioritiesSum int = 0

	for _, item := range itemsRepeatedInAllRucksacks {
		prioritiesSum += strings.Index(possibleItems, item) + 1
	}

	fmt.Println(prioritiesSum)
}

// partTwo solves the second part of the Advent of Code day 03.
// Find the item type that corresponds to the badges of each three-Elf group.
// What is the sum of the priorities of those item types?
func partTwo(rucksackItems []string) {
	fmt.Println("Solution for part 2:")

	var badges []string

	for index := 0; index < len(rucksackItems); index += 3 {
		firstElfRucksack := rucksackItems[index]
		secondElfRucksack := rucksackItems[index+1]
		thirdElfRucksack := rucksackItems[index+2]

		for _, item := range firstElfRucksack {
			item := string(item)

			if strings.Contains(secondElfRucksack, item) && strings.Contains(thirdElfRucksack, item) {
				groupBadge := item
				badges = append(badges, groupBadge)
				break
			}
		}
	}

	var possibleItems string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	var prioritiesSum int = 0

	for _, badge := range badges {
		prioritiesSum += strings.Index(possibleItems, badge) + 1
	}

	fmt.Println(prioritiesSum)

}

func contains(splice []string, element string) bool {
	for _, character := range splice {
		if character == element {
			return true
		}
	}
	return false
}

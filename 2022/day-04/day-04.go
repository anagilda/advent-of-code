package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/anagilda/advent-of-code/2022/utils"
)

// main executes the solution to the Advent of Code day 04.
func main() {
	cleaningAssignmentPairs := utils.ReadInput()

	partOne(cleaningAssignmentPairs)
	partTwo(cleaningAssignmentPairs)
}

// partOne solves the first part of the Advent of Code day 04.
// In how many assignment pairs does one range fully contain the other?
func partOne(cleaningAssignmentPairs []string) {
	fmt.Println("Solution for part 1:")

	fullyContainedPairs := 0

	for _, pair := range cleaningAssignmentPairs {
		bothAssignments := strings.Split(pair, ",")
		assignmentElfOne, assignmentElfTwo := bothAssignments[0], bothAssignments[1]

		cleaningRangeElfOne := getCleaningSectionsRange(assignmentElfOne)
		firstSectionElfOne, lastSectionElfOne := cleaningRangeElfOne[0], cleaningRangeElfOne[len(cleaningRangeElfOne)-1]
		cleaningRangeElfTwo := getCleaningSectionsRange(assignmentElfTwo)
		firstSectionElfTwo, lastSectionElfTwo := cleaningRangeElfTwo[0], cleaningRangeElfTwo[len(cleaningRangeElfTwo)-1]

		if firstSectionElfOne >= firstSectionElfTwo && lastSectionElfOne <= lastSectionElfTwo ||
			firstSectionElfTwo >= firstSectionElfOne && lastSectionElfTwo <= lastSectionElfOne {
			fullyContainedPairs += 1
		}

	}

	fmt.Println(fullyContainedPairs)
}

// partTwo solves the second part of the Advent of Code day 04.
func partTwo(cleaningAssignmentPairs []string) {
	fmt.Println("Solution for part 2:")

	overlappingPairs := 0

	for _, pair := range cleaningAssignmentPairs {
		bothAssignments := strings.Split(pair, ",")
		assignmentElfOne, assignmentElfTwo := bothAssignments[0], bothAssignments[1]

		fmt.Printf("%v and %v \n", assignmentElfOne, assignmentElfTwo)

		cleaningRangeElfOne := getCleaningSectionsRange(assignmentElfOne)
		cleaningRangeElfTwo := getCleaningSectionsRange(assignmentElfTwo)

		for _, cleaningSection := range cleaningRangeElfOne {
			if contains(cleaningRangeElfTwo, cleaningSection) {
				overlappingPairs += 1
				break
			}

		}
	}
	fmt.Println(overlappingPairs)
}

func getCleaningSectionsRange(cleaningAssignment string) []int {
	clearingLimits := strings.Split(cleaningAssignment, "-")

	firstSection, err := strconv.Atoi(clearingLimits[0])
	if err != nil {
		fmt.Println(err)
	}
	lastSection, err := strconv.Atoi(clearingLimits[1])
	if err != nil {
		fmt.Println(err)
	}

	limitedRange := createRange(firstSection, lastSection)
	fmt.Println(limitedRange)

	return limitedRange

}

func createRange(minimum int, maximum int) []int {
	limitedRange := make([]int, maximum-minimum+1)

	for number := range limitedRange {
		limitedRange[number] = minimum + number
	}

	return limitedRange
}

func contains(splice []int, element int) bool {
	for _, character := range splice {
		if character == element {
			return true
		}
	}
	return false
}

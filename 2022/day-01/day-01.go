package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

// sum calculates the sum of the values in the provided array.
func sum(array []int) int {
	sumOfValues := 0
	for _, value := range array {
		sumOfValues += value
	}
	return sumOfValues
}

// getCalorieCountPerElfFromInput reads the input file and gets an ordered calorie count per Elf.
// It sums the calories per Elf and then sorts the calorie sums in ascending order.
func getCalorieCountPerElfFromInput() []int {
	fmt.Println("Reading input...")

	inputFile, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
	}
	defer inputFile.Close()

	fileScanner := bufio.NewScanner(inputFile)

	fmt.Println("Summing calories per Elf...")
	calorieCountPerElf := []int{}
	caloriesBroughtByElf := 0

	for fileScanner.Scan() {
		line := fileScanner.Text()

		if line != "" {
			calories, error := strconv.Atoi(line)
			if error != nil {
				fmt.Println(err)
			}

			caloriesBroughtByElf += calories
		} else {
			calorieCountPerElf = append(calorieCountPerElf, caloriesBroughtByElf)
			caloriesBroughtByElf = 0
		}
	}

	// Handle last Elf
	calorieCountPerElf = append(calorieCountPerElf, caloriesBroughtByElf)

	sort.Ints(calorieCountPerElf[:])

	return calorieCountPerElf
}

// partOne solves the first part of the Advent of Code day 01.
// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
func partOne(sortedCalorieCountPerElf []int) {
	fmt.Println("Solution for part 1:")

	highestCalories := sortedCalorieCountPerElf[len(sortedCalorieCountPerElf)-1]

	fmt.Println(highestCalories)
}

// partTwo solves the second part of the Advent of Code day 01.
// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
func partTwo(sortedCalorieCountPerElf []int) {
	fmt.Println("Solution for part 2:")

	numberOfElves := 3
	sumOfCalories := sum(sortedCalorieCountPerElf[len(sortedCalorieCountPerElf)-numberOfElves:])

	fmt.Println(sumOfCalories)
}

// main executes the solution to the Advent of Code day 01.
func main() {
	calorieCountPerElf := getCalorieCountPerElfFromInput()

	partOne(calorieCountPerElf)
	partTwo(calorieCountPerElf)
}

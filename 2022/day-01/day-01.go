package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func findMaxElement(arr []int) int {
	max_num := arr[0]
	for i := 0; i < len(arr); i++ {
		if arr[i] > max_num {
			max_num = arr[i]
		}
	}
	return max_num
}

func sum(array []int) int {
	result := 0
	for _, v := range array {
		result += v
	}
	return result
}

func main() {
	fmt.Println("Reading input...")

	// Open the file
	readFile, err := os.Open("input.txt")
	if err != nil {
		fmt.Println(err)
	}

	// Create the file scanner
	fileScanner := bufio.NewScanner(readFile)

	// Split the file into lines
	fileScanner.Split(bufio.ScanLines)
	var caloryCount []string

	// Get each line and process it
	for fileScanner.Scan() {
		caloryCount = append(caloryCount, fileScanner.Text())
	}

	readFile.Close()

	fmt.Println("Summing calories per Elf...")
	// fmt.Printf("%v", caloryCount)

	var caloryCountPerElf []int
	var caloriesBroughtByElf int = 0

	for _, line := range caloryCount {

		if line != "" {
			// fmt.Println(line)
			calories, error := strconv.Atoi(line)
			if error != nil {
				fmt.Println(err)
			}

			caloriesBroughtByElf = caloriesBroughtByElf + calories

			// fmt.Println(caloriesBroughtByElf)
		} else {
			caloryCountPerElf = append(caloryCountPerElf, caloriesBroughtByElf)
			caloriesBroughtByElf = 0
		}
	}

	// fmt.Printf("%v", caloryCountPerElf)
	// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
	fmt.Println("Solution for part 1:")
	fmt.Println(findMaxElement(caloryCountPerElf))

	// Sort all calories
	sort.Ints(caloryCountPerElf[:])
	// fmt.Printf("%v", caloryCountPerElf)

	var numberOfElves int = 3
	// fmt.Println(caloryCountPerElf[len(caloryCountPerElf)-numberOfElves:])
	var sumOfCalories int = sum(caloryCountPerElf[len(caloryCountPerElf)-numberOfElves:])

	fmt.Println("Solution for part 2:")
	fmt.Printf("%v", sumOfCalories)
}

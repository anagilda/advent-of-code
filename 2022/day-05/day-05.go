package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/anagilda/advent-of-code/2022/utils"
)

// main executes the solution to the Advent of Code day 05.
func main() {

	crateMovingPlan := utils.ReadInput()

	var emptyLineIndex int

	for index, line := range crateMovingPlan {
		if line == "" {
			emptyLineIndex = index
			break
		}
	}

	startingStacksDrawing, rearrangementProcedureInstructions := crateMovingPlan[:emptyLineIndex-1], crateMovingPlan[emptyLineIndex+1:]

	containerStacks := make([][]string, len(startingStacksDrawing)+1)

	for index, _ := range startingStacksDrawing {
		reverseIndex := len(startingStacksDrawing) - 1 - index
		containerRow := startingStacksDrawing[reverseIndex]
		regex := regexp.MustCompile(`^    `)
		containerRow = regex.ReplaceAllString(containerRow, "[-] ")
		containerRow = strings.Replace(containerRow, "    ", " [-]", -1)

		// Fix a bug for the [-][H] from first row where there are really only 4 spaces
		// ...

		containersInRow := strings.Split(containerRow, " ")

		for index, container := range containersInRow {
			if container != "[-]" {
				containerStacks[index] = append(containerStacks[index], container)
			}
		}
	}

	var rearrangementProcedure []ContainerRearrangement
	for _, instruction := range rearrangementProcedureInstructions {

		re := regexp.MustCompile("[0-9]+")
		codeIntruction := re.FindAllString(instruction, -1)

		numberOfContainers, _ := strconv.Atoi(codeIntruction[0])
		fromStack, _ := strconv.Atoi(codeIntruction[1])
		toStack, _ := strconv.Atoi(codeIntruction[2])
		rearrangementInstruction := ContainerRearrangement{numberOfContainers: numberOfContainers, fromStack: fromStack - 1, toStack: toStack - 1}

		rearrangementProcedure = append(rearrangementProcedure, rearrangementInstruction)

	}

	// THIS COPY DOES NOT WORK
	// containerStacksCopy := make([][]string, len(containerStacks))
	// copy(containerStacksCopy, containerStacks)
	// fmt.Println(containerStacks)
	// fmt.Println(containerStacksCopy)

	// Can only run one at a time
	partOne(containerStacks, rearrangementProcedure)
	partTwo(containerStacks, rearrangementProcedure)
}

// partOne solves the first part of the Advent of Code day 05.
// After the rearrangement procedure completes, what crate ends up on top of each stack?
// CrateMover 9000
func partOne(containerStacks [][]string, rearrangementProcedure []ContainerRearrangement) {
	fmt.Println("Solution for part 1:")

	for _, instruction := range rearrangementProcedure {
		// fmt.Println(">>>>>>>>>>>>>>>>>>>>>>>>>>>")
		// fmt.Println(containerStacks[instruction.fromStack])
		// fmt.Println(containerStacks[instruction.toStack])

		// fmt.Println("=====", instruction.numberOfContainers, instruction.fromStack, instruction.toStack)

		for containersMoved := 0; containersMoved < instruction.numberOfContainers; containersMoved++ {
			// fmt.Println(containersMoved)
			containerToMove, updatedFromStack := removeItem(len(containerStacks[instruction.fromStack])-1, containerStacks[instruction.fromStack])
			// fmt.Printf("---%v-\n", containerToMove)

			containerStacks[instruction.fromStack] = updatedFromStack
			containerStacks[instruction.toStack] = append(containerStacks[instruction.toStack], containerToMove)
			// fmt.Println(containerStacks[instruction.fromStack])
			// fmt.Println(containerStacks[instruction.toStack])
			// fmt.Println()
		}
		// time.Sleep(30 * time.Second)

	}

	firstContainersInStacks := ""
	for _, stack := range containerStacks {
		// fmt.Println(stack)
		lastContainerInStack := stack[len(stack)-1]
		lastContainerInStack = strings.ReplaceAll(lastContainerInStack, "[", "")
		lastContainerInStack = strings.ReplaceAll(lastContainerInStack, "]", "")
		firstContainersInStacks += lastContainerInStack
	}
	fmt.Println(firstContainersInStacks)
}

// partTwo solves the second part of the Advent of Code day 05.
// After the rearrangement procedure completes, what crate ends up on top of each stack?
// CrateMover 9001
func partTwo(containerStacks [][]string, rearrangementProcedure []ContainerRearrangement) {
	fmt.Println("Solution for part 2:")
	fmt.Println(containerStacks)

	for _, instruction := range rearrangementProcedure {
		// fmt.Println(">>>>>>>>>>>>>>>>>>>>>>>>>>>")
		// fmt.Println(containerStacks[instruction.fromStack])
		// fmt.Println(containerStacks[instruction.toStack])

		// fmt.Println("=====", instruction.numberOfContainers, instruction.fromStack, instruction.toStack)

		var containersToMove []string

		for containersMoved := 0; containersMoved < instruction.numberOfContainers; containersMoved++ {
			// fmt.Println(containersMoved)
			// fmt.Printf("---%v-\n", containerToMove)

			containerToMove, updatedFromStack := removeItem(len(containerStacks[instruction.fromStack])-1, containerStacks[instruction.fromStack])

			containerStacks[instruction.fromStack] = updatedFromStack
			containersToMove = append([]string{containerToMove}, containersToMove...)

		}
		containerStacks[instruction.toStack] = append(containerStacks[instruction.toStack], containersToMove...)

		// fmt.Println(containerStacks[instruction.fromStack])
		// fmt.Println(containerStacks[instruction.toStack])

	}

	firstContainersInStacks := ""
	for _, stack := range containerStacks {
		// fmt.Println(stack)
		lastContainerInStack := stack[len(stack)-1]
		lastContainerInStack = strings.ReplaceAll(lastContainerInStack, "[", "")
		lastContainerInStack = strings.ReplaceAll(lastContainerInStack, "]", "")
		firstContainersInStacks += lastContainerInStack
	}
	fmt.Print(firstContainersInStacks)
}

type ContainerRearrangement struct {
	numberOfContainers int
	fromStack          int
	toStack            int
}

func removeItem(index int, array []string) (string, []string) {
	removedElement := array[index]
	newArray := append(array[:index], array[index+1:]...)
	return removedElement, newArray
}

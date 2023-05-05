package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/anagilda/advent-of-code/2022/utils"
)

// main executes the solution to the Advent of Code day 08.
func main() {
	treeMap := utils.ReadInput()

	// treeMap = []string{
	// 	"30373",
	// 	"25512",
	// 	"65332",
	// 	"33549",
	// 	"35390",
	// }

	var treeConfiguration [][]string
	for _, row := range treeMap {
		treeConfiguration = append(treeConfiguration, strings.Split(row, ""))
	}

	partOne(treeConfiguration)
	partTwo(treeConfiguration)
}

// partOne solves the first part of the Advent of Code day 08.
func partOne(treeConfiguration [][]string) {
	fmt.Println("Solution for part 1:")
	numberOfRows := len(treeConfiguration)
	numberOfColumns := len(treeConfiguration[0])

	var visibleTrees []Tree
	for columnIndex := 0; columnIndex < numberOfColumns; columnIndex++ {
		rowIndexForEdges := [2]int{0, numberOfRows - 1}

		for _, rowIndex := range rowIndexForEdges {
			treeHeight, _ := strconv.Atoi(treeConfiguration[rowIndex][columnIndex])
			visibleTrees = append(visibleTrees, Tree{row: rowIndex, column: columnIndex, height: treeHeight})

		}
	}
	for rowIndex := 1; rowIndex < numberOfRows-1; rowIndex++ {
		columnIndexForEdges := [2]int{0, numberOfColumns - 1}

		for _, columnIndex := range columnIndexForEdges {
			treeHeight, _ := strconv.Atoi(treeConfiguration[rowIndex][columnIndex])
			visibleTrees = append(visibleTrees, Tree{row: rowIndex, column: columnIndex, height: treeHeight})

		}
	}
	// fmt.Println(visibleTrees)

	for rowIndex := 1; rowIndex < numberOfRows-1; rowIndex++ {

		row := treeConfiguration[rowIndex]
		for columnIndex := 1; columnIndex < numberOfColumns-1; columnIndex++ {
			// fmt.Print(row[columnIndex])
			treeHeight, _ := strconv.Atoi(row[columnIndex])
			tree := Tree{row: rowIndex, column: columnIndex, height: treeHeight}

			// fmt.Println(tree)
			tree.isVisible = isTreeVisible(tree, treeConfiguration)
			if tree.isVisible {
				visibleTrees = append(visibleTrees, tree)
			}

		}
		// fmt.Println(visibleTrees)
		// fmt.Println(len(visibleTrees))

		// if len(visibleTrees) > 100 {
		// 	break
		// }

	}
	// fmt.Println(visibleTrees)
	fmt.Println(len(visibleTrees))

}

// partTwo solves the second part of the Advent of Code day 08.
func partTwo(treeConfiguration [][]string) {
	fmt.Println("Solution for part 2:")

	numberOfRows := len(treeConfiguration)
	numberOfColumns := len(treeConfiguration[0])

	highestScenicScore := 0
	for rowIndex := 0; rowIndex < numberOfRows-1; rowIndex++ {
		for columnIndex := 0; columnIndex < numberOfColumns-1; columnIndex++ {
			treeHeight, _ := strconv.Atoi(treeConfiguration[rowIndex][columnIndex])
			tree := Tree{
				row:    rowIndex,
				column: columnIndex,
				height: treeHeight,
			}
			tree.scenicScore = calculateScenicScore(tree, treeConfiguration)
			// fmt.Println(tree)

			if tree.scenicScore > highestScenicScore {
				highestScenicScore = tree.scenicScore
			}

			if tree.scenicScore == 2772420 {
				fmt.Println(tree)
			}
		}
	}
	fmt.Println(highestScenicScore)
}

func isTreeVisible(tree Tree, treeConfiguration [][]string) bool {

	numberOfRows := len(treeConfiguration)
	numberOfColumns := len(treeConfiguration[0])

	maxHeightOfTreesOnTop := 0
	for rowIndex := 0; rowIndex < tree.row; rowIndex++ {
		treeHeight, _ := strconv.Atoi(treeConfiguration[rowIndex][tree.column])
		if treeHeight > maxHeightOfTreesOnTop {
			maxHeightOfTreesOnTop = treeHeight
		}
	}
	if maxHeightOfTreesOnTop < tree.height {
		// fmt.Printf("=top %v\n", tree)
		return true
	}

	maxHeightOfTreesOnTheBottom := 0
	for rowIndex := numberOfRows - 1; rowIndex > tree.row; rowIndex -= 1 {
		treeHeight, _ := strconv.Atoi(treeConfiguration[rowIndex][tree.column])
		if treeHeight > maxHeightOfTreesOnTheBottom {
			maxHeightOfTreesOnTheBottom = treeHeight
		}
	}
	if maxHeightOfTreesOnTheBottom < tree.height {
		// fmt.Printf("=bottom %v\n", tree)
		return true
	}

	maxHeightOfTreesOnTheLeft := 0
	for columnIndex := 0; columnIndex < tree.column; columnIndex++ {
		treeHeight, _ := strconv.Atoi(treeConfiguration[tree.row][columnIndex])
		if treeHeight > maxHeightOfTreesOnTheLeft {
			maxHeightOfTreesOnTheLeft = treeHeight
		}
	}
	if maxHeightOfTreesOnTheLeft < tree.height {
		// fmt.Printf("=left %v\n", tree)
		return true
	}

	maxHeightOfTreesOnTheRight := 0
	for columnIndex := numberOfColumns - 1; columnIndex > tree.column; columnIndex -= 1 {
		treeHeight, _ := strconv.Atoi(treeConfiguration[tree.row][columnIndex])
		if treeHeight > maxHeightOfTreesOnTheRight {
			maxHeightOfTreesOnTheRight = treeHeight
		}
	}
	if maxHeightOfTreesOnTheRight < tree.height {
		// fmt.Printf("=right %v\n", tree)
		return true
	}

	return false
}

func calculateScenicScore(tree Tree, treeConfiguration [][]string) int {

	numberOfRows := len(treeConfiguration)
	numberOfColumns := len(treeConfiguration[0])

	scenicScoreTop := 0
	for rowIndex := tree.row - 1; rowIndex >= 0; rowIndex -= 1 {
		scenicScoreTop += 1

		treeHeight, _ := strconv.Atoi(treeConfiguration[rowIndex][tree.column])
		if treeHeight >= tree.height {
			break
		}
	}
	// fmt.Printf("=top %v\n", scenicScoreTop)

	scenicScoreBottom := 0
	for rowIndex := tree.row + 1; rowIndex < numberOfRows; rowIndex++ {
		scenicScoreBottom += 1

		treeHeight, _ := strconv.Atoi(treeConfiguration[rowIndex][tree.column])
		if treeHeight >= tree.height {
			break
		}
	}
	// fmt.Printf("=bottom %v\n", scenicScoreBottom)

	scenicScoreLeft := 0
	for columnIndex := tree.column - 1; columnIndex >= 0; columnIndex -= 1 {
		scenicScoreLeft += 1

		treeHeight, _ := strconv.Atoi(treeConfiguration[tree.row][columnIndex])
		if treeHeight >= tree.height {
			break
		}
	}
	// fmt.Printf("=left %v\n", scenicScoreLeft)

	scenicScoreRight := 0
	for columnIndex := tree.column + 1; columnIndex < numberOfColumns; columnIndex++ {
		scenicScoreRight += 1

		treeHeight, _ := strconv.Atoi(treeConfiguration[tree.row][columnIndex])
		if treeHeight >= tree.height {
			break
		}
	}
	// fmt.Printf("=right %v\n", scenicScoreRight)

	scenicScore := scenicScoreTop * scenicScoreBottom * scenicScoreLeft * scenicScoreRight
	// fmt.Printf("ALL %v\n", scenicScore)

	return scenicScore
}

type Tree struct {
	row         int
	column      int
	height      int
	isVisible   bool
	scenicScore int
}

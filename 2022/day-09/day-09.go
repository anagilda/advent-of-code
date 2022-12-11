package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/anagilda/advent-of-code/2022/utils"
)

// main executes the solution to the Advent of Code day 09.
func main() {
	ropeMotions := utils.ReadInput()

	// ropeMotions = []string{
	// 	"R 4",
	// 	"U 4",
	// 	"L 3",
	// 	"D 1",
	// 	"R 4",
	// 	"D 1",
	// 	"L 5",
	// 	"R 2",
	// }

	var headMotions []Move

	for _, motion := range ropeMotions {
		motionSplit := strings.Split(motion, " ")

		steps, _ := strconv.Atoi(motionSplit[1])
		headMotions = append(headMotions, Move{direction: motionSplit[0], steps: steps})
	}

	head := Coordinates{x: 0, y: 0}
	tail := Coordinates{x: 0, y: 0}

	positionsVisitedByTheTail := make(map[Coordinates]struct{})
	type void struct{}
	var visited void

	for _, move := range headMotions {
		sense := 1
		if move.direction == "D" || move.direction == "L" {
			sense = -1
		}

		for step := 0; step < move.steps; step++ {
			fmt.Printf("--- head %v | tail %v , %v\n", head, tail, step)
			initialHeadInStep := head

			if move.direction == "U" || move.direction == "D" {
				head.y += sense * 1
			}
			if move.direction == "L" || move.direction == "R" {
				head.x += sense * 1
			}

			// fmt.Println(head)

			// if step == 0
			if tail == initialHeadInStep {
				positionsVisitedByTheTail[tail] = visited
				continue
			}

			differenceX := head.x - tail.x
			differenceY := head.y - tail.y

			senseX := 1
			senseY := 1
			if differenceX < 0 {
				senseX = -1
			}
			if differenceY < 0 {
				senseY = -1
			}

			// Two points cannot be touching
			if Abs(differenceX) >= 2 && differenceY == 0 {
				tail.x += senseX * 1
			} else if Abs(differenceY) >= 2 && differenceX == 0 {
				tail.y += senseY * 1
			} else if Abs(differenceX)+Abs(differenceY) > 2 {
				tail.x += senseX * 1
				tail.y += senseY * 1
			}
			// if they are both 0, then don't move.
			positionsVisitedByTheTail[tail] = visited
			// fmt.Println(positionsVisitedByTheTail)

		}

	}
	fmt.Printf("--- head %v | tail %v , end\n", head, tail)

	partOne(positionsVisitedByTheTail)
	// partTwo()
}

// partOne solves the first part of the Advent of Code day 09.
// How many positions does the tail of the rope visit at least once?
func partOne(positionsVisitedByTheTail map[Coordinates]struct{}) {
	fmt.Println("Solution for part 1:")
	fmt.Println(len(positionsVisitedByTheTail))
}

// partTwo solves the second part of the Advent of Code day 09.
func partTwo() {
	fmt.Println("Solution for part 2:")
	panic("unimplemented")
}

type Coordinates struct {
	x int
	y int
}

type Move struct {
	direction string
	steps     int
}

func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

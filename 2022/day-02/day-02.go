package main

import (
	"fmt"
	"strings"

	"github.com/anagilda/advent-of-code/2022/utils"
)

// main executes the solution to the Advent of Code day 02.
func main() {
	matches := utils.ReadInput()

	gameByCode := getAllMatchesByCode(matches)
	partOne(gameByCode)

	gameByOutcome := getAllMatchesByOutcome(matches)
	partTwo(gameByOutcome)
}

// partOne solves the first part of the Advent of Code day 02.
// We consider the game result from the perspective of the player 2.
// What would your total score be if everything goes exactly according to your strategy guide?
func partOne(game []Match) {
	fmt.Println("Solution for part 1:")

	score := 0

	for _, match := range game {
		var matchResult Result

		if match.playerOne == match.playerTwo {
			matchResult = Draw
		} else if match.playerOne == Rock && match.playerTwo == Paper ||
			match.playerOne == Paper && match.playerTwo == Scissors ||
			match.playerOne == Scissors && match.playerTwo == Rock {
			matchResult = Win
		} else {
			matchResult = Loss
		}

		matchScore := int(matchResult) + int(match.playerTwo)
		score += matchScore

	}

	fmt.Printf("The total score of the player 2: %v", score)
}

// partTwo solves the second part of the Advent of Code day 02.
// Following the Elf's instructions for the second column,
// what would your total score be if everything goes exactly according to your strategy guide?
func partTwo(game []Match) {
	fmt.Println("Solution for part 2:")

	score := 0

	for _, match := range game {
		var matchResult Result

		if match.playerOne == match.playerTwo {
			matchResult = Draw
		} else if match.playerOne == Rock && match.playerTwo == Paper ||
			match.playerOne == Paper && match.playerTwo == Scissors ||
			match.playerOne == Scissors && match.playerTwo == Rock {
			matchResult = Win
		} else {
			matchResult = Loss
		}

		matchScore := int(matchResult) + int(match.playerTwo)
		score += matchScore

	}

	fmt.Printf("The total score of the player 2: %v", score)
}

// getAllMatchesByCode gets the symbols played for both players in a game of Rock-Paper-Scissors.
// It follows the assumption that each play is coded as a letter for each player.
func getAllMatchesByCode(matches []string) []Match {
	var game []Match
	for _, match := range matches {

		playsInMatch := strings.Fields(match)
		playerOnePlayLetter, playerTwoPlayLetter := playsInMatch[0], playsInMatch[1]

		var playerOnePlay Symbol
		switch playerOnePlayLetter {
		case "A":
			playerOnePlay = Rock
		case "B":
			playerOnePlay = Paper
		case "C":
			playerOnePlay = Scissors
		}

		var playerTwoPlay Symbol
		switch playerTwoPlayLetter {
		case "X":
			playerTwoPlay = Rock
		case "Y":
			playerTwoPlay = Paper
		case "Z":
			playerTwoPlay = Scissors
		}

		// fmt.Printf("%v vs %v —> %v vs %v\n", playerOnePlayLetter, playerTwoPlayLetter, playerOnePlay, playerTwoPlay)
		game = append(game, Match{playerOne: playerOnePlay, playerTwo: playerTwoPlay})
	}

	return game
}

// getAllMatchesByOutput gets the symbols played for both players in a game of Rock-Paper-Scissors.
// It assumes that the play for the player 1 is shown as a letter, and the play for player 2 is guessed based on the outcome of the match.
func getAllMatchesByOutcome(matches []string) []Match {
	var game []Match
	for _, match := range matches {

		playsInMatch := strings.Fields(match)
		playerOnePlayLetter, matchOutcome := playsInMatch[0], playsInMatch[1]

		var playerOnePlay Symbol
		switch playerOnePlayLetter {
		case "A":
			playerOnePlay = Rock
		case "B":
			playerOnePlay = Paper
		case "C":
			playerOnePlay = Scissors
		}

		var playerTwoPlay Symbol
		switch matchOutcome {
		case "X":
			if playerOnePlay == Rock {
				playerTwoPlay = Scissors
			} else if playerOnePlay == Paper {
				playerTwoPlay = Rock
			} else if playerOnePlay == Scissors {
				playerTwoPlay = Paper
			}
		case "Y":
			playerTwoPlay = playerOnePlay
		case "Z":
			if playerOnePlay == Rock {
				playerTwoPlay = Paper
			} else if playerOnePlay == Paper {
				playerTwoPlay = Scissors
			} else if playerOnePlay == Scissors {
				playerTwoPlay = Rock
			}
		}

		// fmt.Printf("%v vs %v —> %v vs %v\n", playerOnePlayLetter, playerTwoPlayLetter, playerOnePlay, playerTwoPlay)
		game = append(game, Match{playerOne: playerOnePlay, playerTwo: playerTwoPlay})
	}

	return game
}

// Symbol is an enum for the symbols possible to play in the Rock-Paper-Scissors game.
type Symbol int

const (
	Rock     Symbol = 1
	Paper           = 2
	Scissors        = 3
)

// Result is an enum for the results possible in a Rock-Paper-Scissors match.
type Result int

const (
	Win  Result = 6
	Draw        = 3
	Loss        = 0
)

// match stores the plays of both players in a Rock-Paper-Scissors match.
type Match struct {
	playerOne Symbol
	playerTwo Symbol
}

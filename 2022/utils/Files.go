package utils

import (
	"bufio"
	"fmt"
	"os"
)

// ReadInput reads a text input file with an expected file name in the current directory.
// It returns an array with the lines of the file.
func ReadInput() []string {
	inputFileName := "input.txt"

	inputFile, err := os.Open(inputFileName)
	if err != nil {
		fmt.Println(err)
	}
	defer inputFile.Close()

	fileScanner := bufio.NewScanner(inputFile)

	var lines []string
	for fileScanner.Scan() {
		lines = append(lines, fileScanner.Text())
	}

	return lines
}

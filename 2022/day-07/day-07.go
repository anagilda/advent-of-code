package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/anagilda/advent-of-code/2022/utils"
)

var partOneResult []int
var partTwoResult *Directory

// main executes the solution to the Advent of Code day 07.
func main() {
	terminalOutput := utils.ReadInput()

	var command string

	var directoryName string

	var fileTree Directory
	var currentDirectory *Directory
	var previousDirectory *Directory
	// {
	// 	'dir': {},
	// 	'dir': {
	// 		'file': 12312,
	// 		'dir': {}
	// 	}
	// }

	for _, line := range terminalOutput {
		// fmt.Println(line)

		if string(line[0]) == "$" {

			command = line[2:]
			// fmt.Println(command)

			commandSections := strings.Split(command, " ")
			if commandSections[0] == "cd" {
				// handle ..
				// handle already exists in tree
				directoryName = commandSections[1]

				if fileTree.name == "" {
					// we start building the tree here
					fileTree = Directory{name: directoryName}
					currentDirectory = &fileTree
				} else if directoryName == ".." {
					// we go back
					previousDirectory = currentDirectory
					currentDirectory = findParentDirectory(&fileTree, previousDirectory, 0)
					// fmt.Printf("Moved back from %v to %v\n", previousDirectory.name, currentDirectory.name)
				} else {
					// we move forward
					previousDirectory = currentDirectory

					for _, dir := range previousDirectory.directories {
						if dir.name == directoryName {
							currentDirectory = dir
							break
						}
					}

				}

			}
			continue
		}

		if command == "ls" {
			outputSections := strings.Split(line, " ")
			if outputSections[0] == "dir" {
				currentDirectory.directories = append(currentDirectory.directories, &Directory{name: outputSections[1]})

			} else {
				fileSize, _ := strconv.Atoi(outputSections[0])
				currentDirectory.files = append(currentDirectory.files, &File{name: outputSections[1], size: fileSize})
			}
		}
	}
	// fmt.Println(fileTree)
	handleDirectory(&fileTree, 0)

	partOne()
	partTwo(fileTree)
}

// partOne solves the first part of the Advent of Code day 07.
func partOne() {
	fmt.Println("Solution for part 1:")
	fmt.Println(sum(partOneResult))
}

// partTwo solves the second part of the Advent of Code day 07.
func partTwo(fileTree Directory) {
	fmt.Println("Solution for part 2:")
	// fmt.Println(fileTree)

	fileSystemSize := 70000000
	updateSize := 30000000
	occupiedSpace := fileTree.totalSize

	freeSpace := fileSystemSize - occupiedSpace
	spaceToFreeUp := updateSize - freeSpace
	// fmt.Printf("Free %v, Occupied %v, Required %v\n", freeSpace, occupiedSpace, spaceToFreeUp)

	if spaceToFreeUp > 0 {
		partTwoResult := findSmallestDirectoryToDelete(&fileTree, spaceToFreeUp)

		fmt.Println(partTwoResult.totalSize)
	} else {
		fmt.Println("There is enough space in the disk")
	}

}

type Directory struct {
	name        string
	directories []*Directory
	files       []*File
	totalSize   int
}

type File struct {
	name string
	size int
}

// handleDirectory draws a tree of the directory and returns its total size.
func handleDirectory(directory *Directory, level int) int {
	indentation := strings.Repeat(" ", level*2)
	fmt.Printf("%v- %v (dir)\n", indentation, directory.name)

	directoryTotalSize := 0

	for _, file := range directory.files {
		indentation := strings.Repeat(" ", (level+1)*2)
		fmt.Printf("%v- %v (file, size=%v)\n", indentation, file.name, file.size)

		directoryTotalSize += file.size
	}

	for _, innerDirectory := range directory.directories {
		directoryTotalSize += handleDirectory(innerDirectory, level+1)
	}

	directory.totalSize = directoryTotalSize

	// partOne
	if directoryTotalSize <= 100000 {
		partOneResult = append(partOneResult, directoryTotalSize)
	}

	return directoryTotalSize

}

func findParentDirectory(fileTree *Directory, directory *Directory, level int) *Directory {
	// fmt.Println(directory.name)
	// fmt.Printf("%v- %v (dir11)\n", "", fileTree.name)

	for _, childDirectory := range fileTree.directories {
		if childDirectory == directory {
			return fileTree
		}

		parentDirectory := findParentDirectory(childDirectory, directory, level+1)
		if parentDirectory != nil {
			return parentDirectory
		}
	}
	if level == 0 {
		panic("Parent directory not found")
	}
	return nil
}

func sum(array []int) int {
	sumOfValues := 0
	for _, value := range array {
		sumOfValues += value
	}
	return sumOfValues
}

func findSmallestDirectoryToDelete(directory *Directory, requiredSpace int) *Directory {
	var smallestDirectoryToDelete *Directory

	if directory.totalSize >= requiredSpace {
		smallestDirectoryToDelete = directory
	}

	for _, dir := range directory.directories {
		childDirectoryToDelete := findSmallestDirectoryToDelete(dir, requiredSpace)

		if childDirectoryToDelete != nil && smallestDirectoryToDelete.totalSize >= childDirectoryToDelete.totalSize {
			smallestDirectoryToDelete = childDirectoryToDelete
			// fmt.Printf("%v\n", smallestDirectoryToDelete.totalSize)
		}
	}

	return smallestDirectoryToDelete
}

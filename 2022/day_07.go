package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type File struct {
	kind     int // 0: Dir, 1: File
	size     int
	parent   *File
	children map[string]*File
}

func create_dir(parent *File) *File {
	return &File{
		kind:     0,
		parent:   parent,
		children: map[string]*File{},
	}
}

func solve(scanner *bufio.Scanner) {
	files := []*File{}
	indegree := map[*File]int{}
	root := create_dir(nil)
	root.parent = root
	node := root
	for scanner.Scan() {
		tokens := strings.Fields(scanner.Text())
		switch tokens[0] {
		case "$":
			switch tokens[1] {
			case "cd":
				switch tokens[2] {
				case "/":
					node = root
				case "..":
					node = node.parent
				default:
					node = node.children[tokens[2]]
				}
			}
		case "dir":
			node.children[tokens[1]] = create_dir(node)
			indegree[node] += 1
		default:
			node.children[tokens[1]] = &File{
				kind:   1,
				size:   str_to_int(tokens[0]),
				parent: node,
			}
			indegree[node] += 1
			files = append(files, node.children[tokens[1]])
		}
	}
	sizes := []int{0}
	for i := 0; i < len(files); i++ {
		file := files[i]
		parent := file.parent
		if parent != file {
			parent.size += file.size
			indegree[parent] -= 1
			if indegree[parent] == 0 {
				files = append(files, parent)
			}
		}
		if file.kind == 0 {
			sizes = append(sizes, file.size)
		}
	}
	sort.Ints(sizes)
	remaining := 70000000 - sizes[len(sizes)-1]
	required := 30000000
	for _, size := range sizes {
		if size+remaining >= required {
			fmt.Println(size)
			return
		}
	}
}

func str_to_int(end string) int {
	v, _ := strconv.ParseInt(end, 10, 64)
	return int(v)
}

var input_files = [2]string{"test.txt", "input.txt"}

func main() {
	for _, input_file := range input_files {
		file, err := os.Open(input_file)
		if err != nil {
			panic(err)
		}
		defer file.Close()
		scanner := bufio.NewScanner(file)
		solve(scanner)
	}
}

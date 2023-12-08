package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type Directions struct {
	left  string
	right string
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)

	directions := ""
	path := make(map[string]Directions)

	starting_possitions_count := 0

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			continue
		}
		if directions == "" {
			directions = line
			continue
		}
		location := line[:3]
		dir := Directions{
			left:  line[7:10],
			right: line[12:15],
		}
		// do something with a line
		path[location] = dir
		if location[2] == 'A' {
			starting_possitions_count++
		}
	}

	positions := make([]string, 0, starting_possitions_count)
	times := make([]int, 0, starting_possitions_count)

	for k := range path {
		if k[2] == 'A' {
			positions = append(positions, k)
			times = append(times, 0)
		}
	}

	i := 0

	finished := false
	zzz := false

	for !finished {
		finished = true
		for j, v := range positions {
			if directions[i%len(directions)] == 'L' {
				positions[j] = path[v].left
			} else {
				positions[j] = path[v].right
			}
			if times[j] == 0 && v[2] == 'Z' {
				times[j] = i
			}
			if !zzz && v == "ZZZ" {
				zzz = true
				fmt.Println(i)
			}
			if times[j] == 0 {
				finished = false
			}
		}
		i++
	}

	fmt.Println(list_LCM(times))

}

func GCD(a int, b int) int {
	if b == 0 {
		return a
	}
	tmp := a
	a = b
	b = tmp % a
	return GCD(a, b)
}

func LCM(a int, b int) int {
	return a * b / GCD(a, b)
}

func list_LCM(list []int) int {
	lcm := list[0]
	for i := 1; i < len(list); i++ {
		lcm = LCM(lcm, list[i])
	}
	return lcm
}

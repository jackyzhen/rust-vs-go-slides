package main

import (
	"fmt"
	"strings"
)

func maxInts(xs []int) int {
	max := xs[0]

	for _, v := range xs {
		if v > max {
			max = v
		}
	}
	return max
}

func maxStrings(xs []string) string {
	max := xs[0]

	for _, v := range xs {
		if strings.Compare(v, max) == 1 {
			max = v
		}
	}
	return max
}

func main() {
	someInts := []int{3, 4, 1, 7, 8, 324}
	someStrings := []string{"aaa", "bbb", "ccc", "ddd"}
	fmt.Printf("%d %s\n", maxInts(someInts), maxStrings(someStrings))
}

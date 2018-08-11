package main

import (
	"fmt"
)

func main() {
	c := make(chan struct{})

	refC := 0
	xs := []int{}
	// m := map[string]string{}

	for i := 0; i < 100; i += 1 {
		go func(num int) {
			refC = refC + 1
			xs = append(xs, num)
			// m["uhoh"] = "picnic"

			c <- struct{}{}
		}(i)
	}

	for i := 0; i < 100; i += 1 {
		<-c
	}

	fmt.Println(refC)    // ???
	fmt.Println(len(xs)) // ???
}

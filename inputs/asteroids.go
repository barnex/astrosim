package main

import (
	"flag"
	"fmt"
	. "math"
	"math/rand"
)

var flagN = flag.Int("n", 1, "number of asteroids")

func main() {
	flag.Parse()

	rmin := 0.5;
	rmax := 1.5;

	for i := 0; i < *flagN; i++ {

		dr := float64(i) / float64(*flagN)
		r := Sqrt(rmin + dr * (rmax - rmin))

		theta := rnd(0.0, 2*Pi)
		v := Sqrt(1 / r)
		x := Cos(theta)
		y := Sin(theta)
		px := r * x
		py := r * y
		vx := v * (-y)
		vy := v * (+x)
		mass := 0.0
		print(mass, px, py, vx, vy)
	}

}

func rnd(min, max float64) float64 {
	return min + rand.Float64()*(max-min)
}

func print(m, x, y, vx, vy float64) {
	fmt.Printf("%.6f,\t%.6f,%.6f,\t%.6f,%.6f\n", m, x, y, vx, vy)
}

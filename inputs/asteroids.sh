#! /bin/bash

cargo run --release --bin astrosim_cli -- \
	-o asteroids.out \
	-t 200 \
	sun.csv asteroids.csv
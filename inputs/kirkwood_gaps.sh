#! /bin/bash

cargo run --release --bin astrosim_cli -- \
	-o kirkwood_gaps.out \
	-t 200 \
	sun.csv jupiter.csv asteroids.csv
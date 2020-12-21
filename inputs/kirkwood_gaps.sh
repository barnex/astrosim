#! /bin/bash

rm kirkwood_gaps.out/density_*.png;

cargo run --release --bin astrosim_cli -- \
	-o kirkwood_gaps.out \
	-t 10000 \
	--render-every 30 \
	--target-error 0.01 \
	sun.csv jupiter.csv asteroids.csv
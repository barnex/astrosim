#! /bin/bash

cargo run --release --bin convergence_adaptive > convergence_adaptive.txt;
cargo run --release --bin convergence_fixed    > convergence_fixed.txt; 

 ./convergence_adaptive.gplot
 ./convergence_fixed.gplot
# BvMap

This is mostly an experiment with using a bitvec to store the occupupied/not occupied
information about each slot in a Vec. It needs nightly rust to compile.

It works well but you can find a more complete implementation in the "stable_vec" crate.
The value of this repo is mostly for the benchmarks comparing all available slotmaps on
crates.io.
Run "cargo bench" to perform the benchmark. Have gnuplot installed to get nice graphs in the report.

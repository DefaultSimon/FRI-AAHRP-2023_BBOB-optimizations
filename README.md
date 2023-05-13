# AAHRP 2023 - Assignment 5: Continuous optimization of BBOB functions

## 1. Preparation
- Clone the repository with `git clone --recurse-submodules git@github.com:DefaultSimon/FRI-AAHRP-2023_BBOB-optimizations.git` (or equivalent).  
  Make sure the `coco` submodule is properly cloned!
- **(before the next two steps are done, any project cargo builds will fail)**
- Execute `cargo install bindgen-cli`.
- Inside the `coco` directory, execute `python3 do.py build-rust` (tested with Python 3.10).
- The `coco-sys` / `coco-rs` libraries are now ready (normal project builds will now work).

## 2. How to validate equality with R's `smoof` package
To validate that the functions we're testing in Rust are actually the same ones as provided by the `smoof`
package in R (`makeBBOBFunction`), a comparison has been set up.

To perform the comparison, run these two scripts:
- Rust: run the project with the `run-R-smoof-comparison` command, e.g. `cargo run -- run-R-smoof-comparison`
- R: run the `smoof_comparison.R` script

Afterward clean up the output as needed and compare the results. The values should match (both scripts round to 4 
decimals to avoid problems with 64-bit float precision).

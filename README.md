<h2 align="center">
    AAHRP 2023, Assignment 5: Continuous optimization of BBOB functions
</h2>
<p align="center">
    Authors: Aljaž Šuštar and Simon Goričar
</p>


## 1. Preparation
> **Important: before the preparation is done, any cargo build will fail due to missing dependencies.**

- Clone the repository with `git clone --recurse-submodules git@github.com:DefaultSimon/FRI-AAHRP-2023_BBOB-optimizations.git` (or equivalent).  
  Make sure the `coco` submodule is properly cloned (if you cloned without the submodules, run `git submodule update --init --recursive`)!
- Execute `cargo install bindgen-cli`.
- Inside the `coco` directory, execute `python3 do.py build-rust` (tested on Python 3.10, no dependencies needed).
- The `coco-sys` / `coco-rs` libraries are now ready.

## 2. How to validate equality with R's `smoof` package
To validate that the functions we're testing in Rust are actually the same ones as provided by the `smoof`
package in R (`makeBBOBFunction`), a comparison has been set up in the `validate_functions` binary.

**To perform the comparison, run the `validate_functions` binary: `cargo run --bin validate_functions`.**
This will automatically find your R installation, run the R comparison script, rerun the same functions in Rust
and perform comparisons of both values.

# AAHRP 2023 - Assignment 5: Continuous optimization of BBOB functions

## 1. Preparation
- Clone the repository with `git clone --recurse-submodules git@github.com:DefaultSimon/FRI-AAHRP-2023_BBOB-optimizations.git` (or equivalent).  
  Make sure the `coco` submodule is properly cloned!
- **(before the next two steps are done, any project cargo builds will fail)**
- Execute `cargo install bindgen-cli`.
- Inside the `coco` directory, execute `python3 do.py build-rust` (tested with Python 3.10).
- The `coco-sys` / `coco-rs` libraries are now ready (normal project builds will now work).

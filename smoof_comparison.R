library("smoof")

print("The output of this script should be compared with the output of `cargo run -- run-R-smoof-comparison`.")
print("Calculating all 24 BBOB functions at point [4, 4, 4, ...] (4 in all 40 dimensions).")
for (functionIndex in 1:24) {
    value <- round(makeBBOBFunction(40, functionIndex, 2023)(rep(4, times = 40)), digits = 4)
    print(sprintf("Function %g: %s", functionIndex, toString(value)))
}
print("Finished.")

# Install the smoof package if not installed yet, then load.
if (!is.element("smoof", installed.packages()[,1])) {
    install.packages("smoof")
}
library("smoof")

print("The output of this script should be compared with the output of `cargo run -- run-R-smoof-comparison`.")
print("Calculating all 24 BBOB functions at point [4, 4, 4, ...] (4 in all 40 dimensions).")

# Calculate the functions and print them out.
print("-- BEGIN --")
for (functionIndex in 1:24) {
    value <- makeBBOBFunction(40, functionIndex, 2023)(rep(4, times = 40))
    print(sprintf("Function %g: %s", functionIndex, toString(value)))
}
print("-- END --")

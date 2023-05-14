# Install the smoof package if not installed yet, then load it.
if (!is.element("smoof", installed.packages()[,1])) {
    install.packages("smoof")
}
library("smoof")

# Configuration and seeding
set.seed(2932921)
config.samplesPerFunction <- 10


cat(
  "Do not run this script standalone, run `cargo run --bin validate_functions` instead.\n"
)
cat(sprintf(
  "Sampling all 24 BBOB functions %.0f times each at random (seeded) points.\n",
  config.samplesPerFunction
))

# Sample the functions and prints results.

cat("-- BEGIN --\n")
for (functionIndex in 1:24) {
    for (sampleIndex in 1:config.samplesPerFunction) {
      inputParameters = runif(40, min = -5, max = 5)
      formattedInputParameters = paste(inputParameters, sep = ",", collapse = ",")
      
      value <- makeBBOBFunction(40, functionIndex, 2023)(inputParameters)
      cat(sprintf(
        "bbob_function_index=%.0f;parameters=[%s];value=%f\n",
        functionIndex, formattedInputParameters, value
      ))
    }
}
cat("-- END --\n")

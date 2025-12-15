Purpose of this project is to centralise my code solutions in Rust for the [Advent of Code](https://adventofcode.com/).

# How to run solutions

Look at `./src` folder to see what years and exercises are available. Then, execute the proper command to resolve any puzzle. For example :

```bash
cargo run 2025 1-1 # Will resolve part 1 of day 1 of year 2025
cargo run 2025 4-2 # Will resolve part 2 of day 4 of year 2025
```

So, expected command is :

```bash
cargo run <YEAR> <DAY>-<PART>
```

# Setup puzzle inputs

By executing the run command only, an error will occur because puzzle input are not present. Each person has a personalized input to resolve when participating to the Advent of Code. For example, to add input for day 1 2025 (both parts), please create the following file `./src/year2025/exo1/exo1.txt` and paste your puzzle input in it.

So, expected input files are :

`./src/year<YEAR>/exo<DAY>/exo<DAY>.txt`

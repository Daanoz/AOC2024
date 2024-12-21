# AOC2024

Advent Of Code 2024, join the fun on the [Advent of code](http://adventofcode.com) website.

The solutions of 2024 will be written in Rust. I will also be trying out to use proc_macro's to load puzzle data and Bevy for rendering puzzles, these might be a bit rough since it's the first time trying to implement these.

## How to run

To automatically download puzzle inputs, set the `AOC_SESSION` environment variable, or pass --aoc-session SESSIONID to the run commands.

All solutions:
```
cargo run --release
```

A single day:
```
cargo run --release -- --day 1
```

Start with bevy renderer:
```
cargo run --release --features render
```

Run benchmarks:
```
cargo bench --bench benchmarks   
cargo bench --bench benchmarks day1/part1  
```

## Solutions

All solutions can be found in the [aoc-solutions-2024/src/solutions](./aoc-solutions-2024/src/solutions/) folder.

## My previous years

- My [*Advent of Code* 2023](https://github.com/daanoz/AOC2023) solutions
- My [*Advent of Code* 2022](https://github.com/daanoz/AOC2022) solutions
- My [*Advent of Code* 2021](https://github.com/daanoz/AOC2021) solutions
- My [*Advent of Code* 2020](https://github.com/daanoz/AOC2020) solutions
- My [*Advent of Code* 2019](https://github.com/daanoz/AOC2019) solutions
- My [*Advent of Code* 2018](https://github.com/daanoz/AOC2018) solutions
- My [*Advent of Code* 2017](https://github.com/daanoz/AOC2017) solutions
- My [*Advent of Code* 2016](https://github.com/daanoz/AOC2016) solutions

## AoC Automation

This code follows the [automation guideline](https://www.reddit.com/r/adventofcode/wiki/faqs/automation).

All inputs and requests are [cached and tagged](./aoc-procmacro-internals/src/fetcher.rs).
<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code {year}

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->

<!--- benchmarking table --->

---

This is a modified and expanded version of [this repository](https://github.com/fspoettel/advent-of-code-rust) from user fspoettel which I am using for my Rust AoC solutions. Instructions for the basic usage and operation of the repository can be found in the `REPO COMMANDS.md` file. 

## Finding Problems, Solutions, and Explanations

For each day, the solution code can be found in `[year number]/src/bin/[day number].rs`. We are asked not to copy/redistribute the problems so there is a link to the original problem at the top of each file. Each year also contains a `solution explanations` folder for explanations of how I solved each problem, which have yet to be filled in.

## Why and How I Altered the Original

The original repository was designed to hold a single year's worth of problems and solutions. When I started working on the 2024 problems I had already done the 2022 problems and I felt that they should go in the same place as a larger body of work. So I set out to expand it to allow for multiple years.

#### The One-Year Problem

The great strength of the original repository are its automated custom commands. A simple `cargo scaffold 2` would create all the necessary files to complete the problems for day 2 and `cargo solve 02` would run the solution. I wanted these commands to continue to work but for obvious organizational purposes, the different years would need to be split into different folders, and the main files for each day would be the same across different years. Cargo did *not* like that. If I simply split it into different year folders within `src/` then cargo would complain about the ambiguity ("which `02.rs` do you want me to run?") There were several other similar issues which, writing this months later, I do not recall. Suffice it to say, there was no easy way to get them to work together.

The solution I found was to use Cargo workspaces, which are intended to allow developing multiple related packages together. While that is not precisely accurate, it allows for each folder for each year to function roughly independently, allowing the custom commands and whatnot to work. Perfect! Except for two massive problems

1) Those commands would only work inside that year's folder, which is a bit of a usability problem. Whenever possible I want to be able to use the custom commands in any folder and have them work as intended.
2) Code outside that workspace (in the main repo or in other workspaces) does not get compiled with the code inside the workspace.

#### The Solution

The solution for problem 1 is relatively simple. The original repo has a `.cargo/config.toml` where the custom commands are defined. I added to it an environment variable `AOC_YEAR` which tracks the year you're currently working on. To set up a new year in the repository I added a `cargo new-year` command which switches `AOC_YEAR` to that year. There is also a new `cargo set-year` command to change it. The custom commands use this variable wherever relevant to determine which year to work with so that the commands are as widely-available as possible.

The solution I ended up going with for problem 2 involves a lot of code duplication; fortunately, it's done automatically. Each year should have all the custom commands, utility code I've written, etc, etc for everything to work perfectly smoothly. So to get it to compile, it's simply duplicated into every year's folder. You'll notice the `year_template/` folder. Whenever you create a new year folder, everything in it is copied from `year_template/`. This means all of the infrastructure code gets duplicated each year. This is an unfortunate tradeoff for smoother usability, but at the very least it's mostly just in the background. The one exception is utility code. If you've written any and you want that to be copied into new years, you have to manually copy it into `year_template/src/utils/` and update it there if you add to or change it.

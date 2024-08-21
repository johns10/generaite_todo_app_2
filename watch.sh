#!/bin/bash

# Run cargo-watch with the following flags:
# -x check: Run 'cargo check' on changes
# -x test: Run 'cargo test' on changes
# -x run: Run the project on changes
# -c: Clear the screen before each run
# -q: Suppress output from cargo-watch itself

cargo watch -x check -x test -x run -c -q

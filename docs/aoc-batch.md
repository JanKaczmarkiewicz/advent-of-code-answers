# Advent of Code batch runner

The runner skips any day with `src/yYYYY/dD/README.md`. For every other day it
downloads the input with `aoc`, solves both parts using the [Fornwall API](https://aoc.fornwall.net/api/), optionally submits the answers, then downloads the README.

Your `aoc-cli` session is read from `~/.adventofcode.session`.

```sh
# Download, solve, and save READMEs for all released years.
python3 scripts/aoc_batch.py

# Submit answers too (README is then downloaded with part two unlocked).
python3 scripts/aoc_batch.py --submit --yes
```

Use `--overwrite` to reprocess a day and `--days 1-5,10` to restrict days.
The script waits five seconds between submissions and retries solver outages
and AoC rate-limit responses.

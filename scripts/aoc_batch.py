#!/usr/bin/env python3
"""Download AoC inputs, solve them with Fornwall, submit, then save READMEs."""

import argparse
import re
import subprocess
import time
import urllib.error
import urllib.request
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SOLVER = "https://aoc.fornwall.workers.dev/solve"
next_submit = 0.0


def aoc(*args):
    result = subprocess.run(["aoc", *args], cwd=ROOT, text=True, capture_output=True)
    if result.returncode:
        raise RuntimeError((result.stdout + result.stderr).strip())
    return result.stdout + result.stderr


def solve(year, day, part, input_file):
    for attempt in range(5):
        request = urllib.request.Request(
            f"{SOLVER}/{year}/{day}/{part}", input_file.read_bytes(),
            headers={"User-Agent": "Mozilla/5.0 (compatible; aoc-batch)"}, method="POST",
        )
        try:
            with urllib.request.urlopen(request, timeout=120) as response:
                return response.read().decode().strip()
        except urllib.error.HTTPError as error:
            if error.code not in (429, 500, 502, 503, 504) or attempt == 4:
                raise RuntimeError(f"solver part {part}: HTTP {error.code}") from error
            wait = int(error.headers.get("Retry-After", 2**attempt))
            print(f"  solver HTTP {error.code}; retrying in {wait}s")
            time.sleep(wait)


def submit(year, day, part, answer, delay):
    global next_submit
    for attempt in range(5):
        time.sleep(max(0, next_submit - time.monotonic()))
        next_submit = time.monotonic() + delay
        output = aoc("submit", "--year", str(year), "--day", str(day), str(part), answer, "--quiet")
        if "right answer" in output.lower() or "already complete" in output.lower() or "already solved" in output.lower():
            return
        match = re.search(r"(\d+)\s*(?:s|seconds?)\s+left to wait", output, re.I)
        if not match or attempt == 4:
            raise RuntimeError(f"submission part {part} failed:\n{output.strip()}")
        wait = int(match.group(1)) + 1
        print(f"  AoC rate limit; retrying in {wait}s")
        time.sleep(wait)


def days(value):
    result = set()
    for group in value.split(","):
        first, _, last = group.partition("-")
        result.update(range(int(first), int(last or first) + 1))
    if not result or min(result) < 1 or max(result) > 25:
        raise argparse.ArgumentTypeError("days must be between 1 and 25")
    return sorted(result)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--from-year", type=int, default=2015)
    parser.add_argument("--to-year", type=int, default=2025)
    parser.add_argument("--days", type=days, default=days("1-25"))
    parser.add_argument("--submit", action="store_true")
    parser.add_argument("--yes", action="store_true", help="required with --submit")
    parser.add_argument("--overwrite", action="store_true")
    parser.add_argument("--submit-delay", type=float, default=5.0)
    args = parser.parse_args()
    if args.submit and not args.yes:
        parser.error("--submit requires --yes")

    for year in range(args.from_year, args.to_year + 1):
        for day in args.days:
            folder = ROOT / "src" / f"y{year}" / f"d{day}"
            readme, input_file = folder / "README.md", folder / "input"
            if readme.exists() and not args.overwrite:
                print(f"{year} day {day:02}: skipped")
                continue
            try:
                folder.mkdir(parents=True, exist_ok=True)
                if args.overwrite or not input_file.exists():
                    aoc("download", "--year", str(year), "--day", str(day), "--input-only", "--input-file", str(input_file), "--quiet")
                answers = [solve(year, day, part, input_file) for part in (1, 2)]
                if args.submit:
                    for part, answer in enumerate(answers, 1):
                        submit(year, day, part, answer, args.submit_delay)
                aoc("download", "--year", str(year), "--day", str(day), "--puzzle-only", "--puzzle-file", str(readme), "--overwrite", "--quiet")
                print(f"{year} day {day:02}: done")
            except RuntimeError as error:
                print(f"{year} day {day:02}: ERROR: {error}")


if __name__ == "__main__":
    main()

import json
import sys
from pathlib import Path

from common import Cargo, binary_artifacts

LAST_SIZE_FILE = Path("target") / "release" / "size.txt"


def measure() -> int:
    """Build the release binary and return its size in bytes.

    Raise CalledProcessError with captured diagnostics if the build fails.
    Exit with a diagnostic if Cargo reports no binary artifact.
    """

    build = Cargo.build(release=True, capture=True)
    messages = (json.loads(line) for line in build.stdout.splitlines())

    for binary in binary_artifacts(messages, "crsh"):
        return binary.stat().st_size

    sys.exit("Cargo did not report a binary artifact")


def percent_diff(size: int) -> float | None:
    """Checks the record from the last run, calculates the percentage difference in size.

    Returns `None` if the record fails to parse or doesn't exist."""

    try:
        with open(LAST_SIZE_FILE, mode="r") as file:
            last = int(file.read().strip())
            diff = float(size - last)
            return round(diff / float(last) * 100, 2)
    except:  # noqa: E722
        return None


def run() -> None:
    """Captures the final size from building the release binary, compares to the last run, and prints results to the terminal."""

    size = measure()
    string = f"{size / 1000 / 1000:.2f}".rstrip("0").rstrip(".") + " MB"

    if (percent := percent_diff(size)) is not None:
        if percent > 0.0:
            string += f" (+{percent}%)"
        elif percent < 0.0:
            string += f" ({percent}%)"
        else:
            string += " (no change)"

    with open(LAST_SIZE_FILE, mode="w") as file:
        file.write(str(size))

    print(string)

import subprocess
import tempfile
import time
from collections.abc import Callable, Generator, Iterable
from pathlib import Path
from typing import Any


def binary_artifacts(messages: Iterable[dict[str, Any]], name: str) -> Generator[Path]:
    """Yield executable paths for the named binary from Cargo JSON messages.

    Reject malformed artifact targets and executable paths with TypeError and
    ValueError respectively.
    """

    for message in messages:
        if message.get("reason") != "compiler-artifact":
            continue

        target = message.get("target")

        if not isinstance(target, dict) or not isinstance(target.get("kind"), list):
            raise TypeError("invalid artifact target")

        if target.get("name") == name and "bin" in target["kind"]:
            executable = message.get("executable")

            if executable is not None and not isinstance(executable, str):
                raise ValueError("invalid executable")

            if executable:
                yield Path(executable)


class SubprocessHelper:
    """Run commands, prepending a subclass's binary when one is defined."""

    BINARY: str = ""

    @classmethod
    def _call(
        cls,
        args: Iterable[str],
        *,
        working_dir: Path | None = None,
        timeout: float | None = None,
        checked: bool = True,
        capture: bool = False,
        on_tick: Callable[[subprocess.Popen[bytes], float], None] | None = None,
        broken_stdout: bool = False,
    ) -> subprocess.CompletedProcess[str]:
        """Run arguments with inherited output, or capture full decoded output.

        Capture uses temporary files and always reaps the child.
        Poll callbacks and broken stdout simulation require capture to be enabled.
        """
        command = [cls.BINARY, *args] if cls.BINARY else list(args)
        if not capture:
            if on_tick is not None or broken_stdout:
                raise ValueError("callbacks and broken stdout require capture=True")
            return subprocess.run(
                command,
                cwd=working_dir,
                timeout=timeout,
                check=checked,
                stdin=subprocess.DEVNULL,
                text=True,
            )
        with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
            process = subprocess.Popen(
                command,
                cwd=working_dir,
                stdin=subprocess.DEVNULL,
                stdout=subprocess.PIPE if broken_stdout else stdout,
                stderr=stderr,
            )
            if broken_stdout:
                assert process.stdout is not None
                process.stdout.close()
            start = time.monotonic()
            try:
                while process.poll() is None:
                    elapsed = time.monotonic() - start
                    if timeout is not None and elapsed >= timeout:
                        raise subprocess.TimeoutExpired(command, timeout)
                    if on_tick is not None:
                        on_tick(process, elapsed)
                    time.sleep(0.01)
            except BaseException as error:
                status_before_cleanup = process.poll()
                if status_before_cleanup is None:
                    process.kill()
                process.wait()
                stdout.seek(0)
                stderr.seek(0)
                output, diagnostic = stdout.read(), stderr.read()
                # Preserve exception-time status separately from the fallback
                # kill/reap. Callers may project these facts without raw notes.
                error.child_capture = {  # ty: ignore[unresolved-attribute]
                    "pid": process.pid,
                    "elapsed_ms": round((time.monotonic() - start) * 1000),
                    "status_before_cleanup": status_before_cleanup,
                    "exit_status": process.returncode,
                    "killed_for_cleanup": status_before_cleanup is None,
                    "reaped": True,
                    "stdout": output.decode("utf-8", "replace"),
                    "stderr": diagnostic.decode("utf-8", "replace"),
                }
                if isinstance(error, subprocess.TimeoutExpired):
                    error.output, error.stderr = output, diagnostic
                else:
                    for name, data in (("stdout", output), ("stderr", diagnostic)):
                        if data:
                            error.add_note(f"{name}:\n{data.decode('utf-8', 'replace')}")
                raise
            finally:
                process.wait()
            stdout.seek(0)
            stderr.seek(0)
            result = subprocess.CompletedProcess(
                command,
                process.returncode,
                stdout.read().decode("utf-8", "replace"),
                stderr.read().decode("utf-8", "replace"),
            )
            result.child_pid = process.pid  # ty: ignore[unresolved-attribute]
            if checked:
                result.check_returncode()
            return result


class Cargo(SubprocessHelper):
    """Query Cargo and build workspace packages with consistent defaults."""

    BINARY: str = "cargo"

    @staticmethod
    def version() -> str:
        """Return Cargo's version string."""
        return Cargo._call(("--version",), capture=True).stdout.strip()

    @staticmethod
    def build(
        project: str | None = None,
        bin: str | None = None,
        target: str | None = None,
        target_dir: Path | None = None,
        working_dir: Path | None = None,
        lib: bool = False,
        features: Iterable[str] = (),
        quiet: bool = True,
        locked: bool = True,
        release: bool = False,
        json_format: bool = True,
        timeout: int | None = None,
        checked: bool = True,
        capture: bool = False,
    ) -> subprocess.CompletedProcess[str]:
        """Build a package, optionally selecting its target, profile, and output format."""
        if lib and bin is not None:
            raise ValueError("cannot build bin and lib in one call")

        args = ["build"]

        if project is not None:
            args.extend(("-p", project))

        if lib:
            args.append("--lib")

        if bin is not None:
            args.extend(("--bin", bin))

        enabled_features = ",".join(features)
        if enabled_features:
            args.extend(("--features", enabled_features))

        if target is not None:
            args.extend(("--target", target))

        if target_dir is not None:
            args.extend(("--target-dir", str(target_dir)))

        if release:
            args.append("--release")

        if quiet:
            args.append("--quiet")

        if locked:
            args.append("--locked")

        if json_format:
            args.append("--message-format=json-render-diagnostics")

        return Cargo._call(
            args, working_dir=working_dir, timeout=timeout, checked=checked, capture=capture
        )

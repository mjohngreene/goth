#!/usr/bin/env python3
"""
Goth Benchmark Test Runner

Runs test cases against the Goth interpreter and reports results.

Usage:
    python run_tests.py                    # Run all tests
    python run_tests.py --category basic   # Run specific category
    python run_tests.py --verbose          # Show all output
    python run_tests.py --json             # Output JSON results
"""

import json
import subprocess
import sys
import os
import argparse
from pathlib import Path
from dataclasses import dataclass, field
from typing import Optional, Any
import math

@dataclass
class TestResult:
    name: str
    passed: bool
    expected: Any
    actual: Optional[str]
    error: Optional[str] = None

@dataclass
class CategoryResult:
    name: str
    passed: int = 0
    failed: int = 0
    results: list = field(default_factory=list)

def find_goth_binary():
    """Find the goth binary."""
    # Check common locations
    candidates = [
        Path(__file__).parent.parent / "crates" / "target" / "release" / "goth",
        Path(__file__).parent.parent / "crates" / "target" / "debug" / "goth",
        Path("goth"),  # In PATH
    ]
    for candidate in candidates:
        if candidate.exists() or (candidate.name == "goth" and subprocess.run(["which", "goth"], capture_output=True).returncode == 0):
            return str(candidate)
    return None

def run_goth(goth_binary: str, file_path: str, args: list) -> tuple[Optional[str], Optional[str]]:
    """Run goth with given file and arguments."""
    cmd = [goth_binary, str(Path(__file__).parent.parent / file_path)] + [str(a) for a in args]
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        if result.returncode == 0:
            return result.stdout.strip(), None
        else:
            return None, result.stderr.strip() or f"Exit code {result.returncode}"
    except subprocess.TimeoutExpired:
        return None, "Timeout"
    except Exception as e:
        return None, str(e)

def compare_output(expected: Any, actual: str, reltol: float = None, abstol: float = None) -> bool:
    """Compare expected and actual output.

    For floating-point, uses relative tolerance (default 1e-9) and absolute tolerance (default 1e-12).
    """
    if actual is None:
        return False

    # Handle boolean outputs (check type explicitly to avoid 1.0 == True issues)
    if isinstance(expected, bool) or expected in ["⊤", "⊥"]:
        expected_bool = expected in ["⊤", True]
        if expected in ["⊥", False]:
            return actual.strip() in ["⊥", "false", "False", "0"]
        return actual.strip() in ["⊤", "true", "True", "1"]

    # Handle numeric outputs
    try:
        actual_num = float(actual.strip())
        expected_num = float(expected)

        # Use relative tolerance for floats (like math.isclose)
        rel = reltol if reltol is not None else 1e-9
        abs_tol = abstol if abstol is not None else 1e-12

        # math.isclose: |a - b| <= max(rel_tol * max(|a|, |b|), abs_tol)
        return math.isclose(actual_num, expected_num, rel_tol=rel, abs_tol=abs_tol)
    except (ValueError, TypeError):
        pass

    # String comparison
    return str(expected).strip() == actual.strip()

def run_test_case(goth_binary: str, test: dict, case: dict, verbose: bool) -> TestResult:
    """Run a single test case."""
    file_path = test["file"]
    inputs = case["input"]
    expected = case["expected"]
    # Support both new names (reltol/abstol) and legacy (tolerance as abstol)
    reltol = case.get("reltol")
    abstol = case.get("abstol") or case.get("tolerance")

    actual, error = run_goth(goth_binary, file_path, inputs)

    if error:
        passed = False
    else:
        passed = compare_output(expected, actual, reltol, abstol)

    if verbose:
        status = "✓" if passed else "✗"
        print(f"  {status} {test['name']}({', '.join(map(str, inputs))}) = {actual or error} (expected {expected})")

    return TestResult(
        name=f"{test['name']}({', '.join(map(str, inputs))})",
        passed=passed,
        expected=expected,
        actual=actual,
        error=error
    )

def run_category(goth_binary: str, test_file: Path, verbose: bool) -> CategoryResult:
    """Run all tests in a category."""
    with open(test_file) as f:
        data = json.load(f)

    category = data["category"]
    result = CategoryResult(name=category)

    if verbose:
        print(f"\n=== {category.upper()} ===")

    for test in data["tests"]:
        for case in test["cases"]:
            test_result = run_test_case(goth_binary, test, case, verbose)
            result.results.append(test_result)
            if test_result.passed:
                result.passed += 1
            else:
                result.failed += 1

    return result

def main():
    parser = argparse.ArgumentParser(description="Run Goth benchmark tests")
    parser.add_argument("--category", "-c", help="Run specific category only")
    parser.add_argument("--verbose", "-v", action="store_true", help="Show detailed output")
    parser.add_argument("--json", "-j", action="store_true", help="Output JSON results")
    parser.add_argument("--goth", help="Path to goth binary")
    args = parser.parse_args()

    # Find goth binary
    goth_binary = args.goth or find_goth_binary()
    if not goth_binary:
        print("Error: Could not find goth binary. Use --goth to specify path.", file=sys.stderr)
        sys.exit(1)

    # Find test files
    test_dir = Path(__file__).parent / "tests"
    if args.category:
        test_files = [test_dir / f"{args.category}.json"]
        if not test_files[0].exists():
            print(f"Error: Category '{args.category}' not found", file=sys.stderr)
            sys.exit(1)
    else:
        test_files = sorted(test_dir.glob("*.json"))

    # Run tests
    all_results = []
    total_passed = 0
    total_failed = 0

    for test_file in test_files:
        result = run_category(goth_binary, test_file, args.verbose)
        all_results.append(result)
        total_passed += result.passed
        total_failed += result.failed

    # Output results
    if args.json:
        output = {
            "summary": {
                "total": total_passed + total_failed,
                "passed": total_passed,
                "failed": total_failed,
                "pass_rate": total_passed / (total_passed + total_failed) if (total_passed + total_failed) > 0 else 0
            },
            "categories": [
                {
                    "name": r.name,
                    "passed": r.passed,
                    "failed": r.failed,
                    "tests": [
                        {
                            "name": t.name,
                            "passed": t.passed,
                            "expected": t.expected,
                            "actual": t.actual,
                            "error": t.error
                        }
                        for t in r.results
                    ]
                }
                for r in all_results
            ]
        }
        print(json.dumps(output, indent=2))
    else:
        print(f"\n{'='*50}")
        print("SUMMARY")
        print(f"{'='*50}")
        for result in all_results:
            status = "✓" if result.failed == 0 else "✗"
            print(f"{status} {result.name}: {result.passed}/{result.passed + result.failed} passed")
        print(f"{'='*50}")
        print(f"TOTAL: {total_passed}/{total_passed + total_failed} passed ({100*total_passed/(total_passed+total_failed):.1f}%)")

        if total_failed > 0:
            print(f"\nFailed tests:")
            for result in all_results:
                for t in result.results:
                    if not t.passed:
                        print(f"  ✗ {t.name}: expected {t.expected}, got {t.actual or t.error}")
            sys.exit(1)

if __name__ == "__main__":
    main()

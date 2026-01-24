#!/usr/bin/env python3
"""
Generate benchmark prompts for LLM testing.

Usage:
    python prompt_generator.py --format syntax factorial
    python prompt_generator.py --format json gcd
    python prompt_generator.py --all --format syntax > prompts.txt
"""

import json
import argparse
from pathlib import Path

# Test specifications for prompt generation
TESTS = {
    "factorial": {
        "signature": "I64 → I64",
        "description": "Compute n! (n factorial). Return 1 for n ≤ 1.",
        "examples": [
            ("factorial(0)", "1"),
            ("factorial(5)", "120"),
            ("factorial(10)", "3628800"),
        ]
    },
    "fibonacci": {
        "signature": "I64 → I64",
        "description": "Compute the nth Fibonacci number. F(0)=0, F(1)=1, F(n)=F(n-1)+F(n-2).",
        "examples": [
            ("fibonacci(0)", "0"),
            ("fibonacci(1)", "1"),
            ("fibonacci(10)", "55"),
        ]
    },
    "gcd": {
        "signature": "I64 → I64 → I64",
        "description": "Compute greatest common divisor using Euclidean algorithm. gcd(a,0)=a, gcd(a,b)=gcd(b, a mod b).",
        "examples": [
            ("gcd(48, 18)", "6"),
            ("gcd(17, 13)", "1"),
        ]
    },
    "isPrime": {
        "signature": "I64 → Bool",
        "description": "Return true (⊤) if n is prime, false (⊥) otherwise. Numbers less than 2 are not prime.",
        "examples": [
            ("isPrime(2)", "⊤"),
            ("isPrime(17)", "⊤"),
            ("isPrime(15)", "⊥"),
            ("isPrime(1)", "⊥"),
        ]
    },
    "power": {
        "signature": "I64 → I64 → I64",
        "description": "Compute base^exp (integer exponentiation). base^0 = 1.",
        "examples": [
            ("power(2, 0)", "1"),
            ("power(2, 10)", "1024"),
            ("power(3, 4)", "81"),
        ]
    },
    "sum_to_n": {
        "signature": "I64 → I64",
        "description": "Compute the sum 1 + 2 + ... + n. Return 0 for n ≤ 0.",
        "examples": [
            ("sum_to_n(0)", "0"),
            ("sum_to_n(10)", "55"),
            ("sum_to_n(100)", "5050"),
        ]
    },
    "abs": {
        "signature": "I64 → I64",
        "description": "Compute absolute value of n.",
        "examples": [
            ("abs(5)", "5"),
            ("abs(-5)", "5"),
            ("abs(0)", "0"),
        ]
    },
    "max_two": {
        "signature": "I64 → I64 → I64",
        "description": "Return the maximum of two numbers.",
        "examples": [
            ("max_two(3, 7)", "7"),
            ("max_two(7, 3)", "7"),
            ("max_two(-3, -7)", "-3"),
        ]
    },
    "isqrt": {
        "signature": "I64 → I64",
        "description": "Compute integer square root (floor of √n).",
        "examples": [
            ("isqrt(0)", "0"),
            ("isqrt(16)", "4"),
            ("isqrt(50)", "7"),
        ]
    },
    "digit_sum": {
        "signature": "I64 → I64",
        "description": "Compute sum of digits in base 10.",
        "examples": [
            ("digit_sum(0)", "0"),
            ("digit_sum(123)", "6"),
            ("digit_sum(9999)", "36"),
        ]
    },
}

SYNTAX_TEMPLATE = """Implement the following function in Goth:

**Function**: {name}
**Signature**: {signature}
**Description**: {description}
**Examples**:
{examples}

Goth syntax notes:
- Function definition: ╭─ name : Type → Type ╰─ body
- De Bruijn indices: ₀ = last param, ₁ = second-to-last, etc.
- For f : A → B → C: ₁ = first arg (A), ₀ = second arg (B)
- Operators: +, -, ×, /, %, =, <, >, ≤, ≥, ∧, ∨, ¬
- Booleans: ⊤ (true), ⊥ (false)
- Conditionals: if cond then e1 else e2

Generate only the Goth code, no explanation."""

JSON_TEMPLATE = """Generate a Goth JSON AST for the following function:

**Function**: {name}
**Signature**: {signature}
**Description**: {description}
**Examples**:
{examples}

JSON AST structure:
- Module: {{"decls": [...]}}
- Function: {{"Fn": {{"name": "main", "signature": ..., "body": ...}}}}
- Types: {{"Base": "I64"}}, {{"Arrow": [arg, ret]}}
- Expressions:
  - Literal: {{"Lit": {{"Int": 42}}}}
  - Index: {{"Idx": 0}} for ₀, {{"Idx": 1}} for ₁
  - App: {{"App": [func, arg]}}
  - BinOp: {{"BinOp": ["Add", left, right]}}
  - If: {{"If": [cond, then, else]}}
  - Name: {{"Name": "main"}}

De Bruijn: For f : A → B → C, Idx(1) = first arg, Idx(0) = second arg.

Output only valid JSON, no explanation."""

def format_examples(examples):
    return "\n".join(f"  {call} = {result}" for call, result in examples)

def generate_prompt(name: str, format: str) -> str:
    if name not in TESTS:
        raise ValueError(f"Unknown test: {name}")

    test = TESTS[name]
    examples = format_examples(test["examples"])

    if format == "syntax":
        return SYNTAX_TEMPLATE.format(
            name=name,
            signature=test["signature"],
            description=test["description"],
            examples=examples
        )
    elif format == "json":
        return JSON_TEMPLATE.format(
            name=name,
            signature=test["signature"],
            description=test["description"],
            examples=examples
        )
    else:
        raise ValueError(f"Unknown format: {format}")

def main():
    parser = argparse.ArgumentParser(description="Generate Goth benchmark prompts")
    parser.add_argument("test", nargs="?", help="Test name (e.g., factorial)")
    parser.add_argument("--format", "-f", choices=["syntax", "json"], default="syntax",
                        help="Output format (syntax or json)")
    parser.add_argument("--all", "-a", action="store_true", help="Generate all prompts")
    parser.add_argument("--list", "-l", action="store_true", help="List available tests")
    args = parser.parse_args()

    if args.list:
        print("Available tests:")
        for name, spec in TESTS.items():
            print(f"  {name}: {spec['signature']}")
        return

    if args.all:
        for name in TESTS:
            print(f"\n{'='*60}")
            print(f"TEST: {name}")
            print(f"{'='*60}\n")
            print(generate_prompt(name, args.format))
    elif args.test:
        print(generate_prompt(args.test, args.format))
    else:
        parser.print_help()

if __name__ == "__main__":
    main()

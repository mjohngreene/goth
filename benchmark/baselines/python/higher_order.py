#!/usr/bin/env python3
"""Python baseline implementations for higher-order functions."""
from functools import reduce

def fold_sum(n: int) -> int:
    """Sum 1..n via fold/reduce"""
    return reduce(lambda a, b: a + b, range(1, n + 1), 0)

def fold_product(n: int) -> int:
    """Product 1..n via fold/reduce (factorial)"""
    return reduce(lambda a, b: a * b, range(1, n + 1), 1)

def compose_square_double(n: int) -> int:
    """Compose: square(double(n))"""
    def double(x): return x * 2
    def square(x): return x * x
    return square(double(n))

def apply_twice_double(n: int) -> int:
    """Apply double twice: double(double(n))"""
    def double(x): return x * 2
    return double(double(n))

def count_evens(n: int) -> int:
    """Count even numbers in 1..n"""
    return len([x for x in range(1, n + 1) if x % 2 == 0])

def pipeline_sum_sq_evens(n: int) -> int:
    """Sum of squares of even numbers in 1..n"""
    evens = [x for x in range(1, n + 1) if x % 2 == 0]
    squares = [x * x for x in evens]
    return sum(squares)


if __name__ == "__main__":
    assert fold_sum(10) == 55
    assert fold_product(5) == 120
    assert compose_square_double(3) == 36
    assert apply_twice_double(3) == 12
    assert count_evens(10) == 5
    assert pipeline_sum_sq_evens(6) == 56  # 2²+4²+6² = 4+16+36
    print("All higher-order tests passed!")

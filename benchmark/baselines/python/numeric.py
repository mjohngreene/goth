#!/usr/bin/env python3
"""Python baseline implementations for numeric functions."""
import math

def gamma_fact(n: float) -> float:
    """Factorial via gamma function: Γ(n+1) = n!"""
    return math.gamma(n + 1)

def sum_squares(n: int) -> int:
    """Sum of squares: 1² + 2² + ... + n²"""
    total = 0
    for i in range(1, n + 1):
        total += i * i
    return total

def product_range(n: int) -> int:
    """Product of 1 to n (factorial)"""
    result = 1
    for i in range(1, n + 1):
        result *= i
    return result

def harmonic(n: int) -> float:
    """Harmonic number H(n) = 1 + 1/2 + 1/3 + ... + 1/n"""
    total = 0.0
    for i in range(1, n + 1):
        total += 1.0 / i
    return total

def exp_taylor(x: float, terms: int = 20) -> float:
    """e^x via Taylor series"""
    result = 0.0
    factorial = 1
    power = 1.0
    for n in range(terms):
        result += power / factorial
        power *= x
        factorial *= (n + 1)
    return result

def sqrt_newton(s: float, iterations: int = 15) -> float:
    """Square root via Newton-Raphson"""
    if s <= 0:
        return 0.0
    guess = s
    for _ in range(iterations):
        guess = (guess + s / guess) / 2
    return guess


if __name__ == "__main__":
    assert abs(gamma_fact(5) - 120) < 0.01
    assert sum_squares(5) == 55
    assert product_range(5) == 120
    assert abs(harmonic(10) - 2.9289682539) < 0.001
    assert abs(exp_taylor(1.0) - 2.718281828) < 0.001
    assert abs(sqrt_newton(2.0) - 1.414213562) < 0.001
    print("All numeric tests passed!")

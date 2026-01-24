#!/usr/bin/env python3
"""Python baseline implementations for recursive functions."""

def factorial(n: int) -> int:
    if n < 2:
        return 1
    return n * factorial(n - 1)

def fibonacci(n: int) -> int:
    if n < 2:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

def sum_to_n(n: int) -> int:
    if n < 1:
        return 0
    return n + sum_to_n(n - 1)

def power(base: int, exp: int) -> int:
    if exp == 0:
        return 1
    return base * power(base, exp - 1)

def gcd(a: int, b: int) -> int:
    if b == 0:
        return a
    return gcd(b, a % b)

def lcm(a: int, b: int) -> int:
    return (a * b) // gcd(a, b)

def ackermann(m: int, n: int) -> int:
    if m == 0:
        return n + 1
    elif n == 0:
        return ackermann(m - 1, 1)
    else:
        return ackermann(m - 1, ackermann(m, n - 1))

def mccarthy91(n: int) -> int:
    if n > 100:
        return n - 10
    return mccarthy91(mccarthy91(n + 11))

def digit_sum(n: int) -> int:
    if n == 0:
        return 0
    return (n % 10) + digit_sum(n // 10)

def tak(x: int, y: int, z: int) -> int:
    if x <= y:
        return z
    return tak(tak(x - 1, y, z), tak(y - 1, z, x), tak(z - 1, x, y))


if __name__ == "__main__":
    assert factorial(5) == 120
    assert fibonacci(10) == 55
    assert sum_to_n(10) == 55
    assert power(2, 10) == 1024
    assert gcd(48, 18) == 6
    assert lcm(4, 6) == 12
    assert ackermann(3, 4) == 125
    assert mccarthy91(99) == 91
    assert digit_sum(123) == 6
    assert tak(18, 12, 6) == 7
    print("All recursion tests passed!")

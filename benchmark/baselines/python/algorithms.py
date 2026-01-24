#!/usr/bin/env python3
"""Python baseline implementations for algorithm functions."""

def is_prime(n: int) -> bool:
    if n < 2:
        return False
    d = 2
    while d * d <= n:
        if n % d == 0:
            return False
        d += 1
    return True

def count_primes(n: int) -> int:
    count = 0
    for i in range(2, n + 1):
        if is_prime(i):
            count += 1
    return count

def nth_prime(n: int) -> int:
    count = 0
    candidate = 2
    while True:
        if is_prime(candidate):
            count += 1
            if count == n:
                return candidate
        candidate += 1

def isqrt(n: int) -> int:
    if n < 0:
        raise ValueError("Cannot compute sqrt of negative number")
    k = 0
    while (k + 1) * (k + 1) <= n:
        k += 1
    return k

def modpow(base: int, exp: int, mod: int) -> int:
    if exp == 0:
        return 1
    if exp % 2 == 0:
        half = modpow(base, exp // 2, mod)
        return (half * half) % mod
    return (base * modpow(base, exp - 1, mod)) % mod

def binary_search(hi: int, target: int) -> int:
    lo = 0
    while lo <= hi:
        mid = (lo + hi) // 2
        if mid == target:
            return mid
        elif mid > target:
            hi = mid - 1
        else:
            lo = mid + 1
    return -1


if __name__ == "__main__":
    assert is_prime(17) == True
    assert is_prime(15) == False
    assert count_primes(20) == 8
    assert nth_prime(10) == 29
    assert isqrt(50) == 7
    assert modpow(2, 10, 1000) == 24
    assert binary_search(10, 7) == 7
    assert binary_search(10, 15) == -1
    print("All algorithm tests passed!")

import gmpy2


def fermat_factorization_attack(n):
    x = gmpy2.isqrt(n)
    y = gmpy2.square(x) - n

    while True:
        x += 1
        y = gmpy2.square(x) - n

        if gmpy2.is_square(y):
            break

    return int(x + gmpy2.isqrt(y)), int(x - gmpy2.isqrt(y))


if __name__ == "__main__":
    n = int(input('n: '))

    p, q = fermat_factorization_attack(n)

    print(f'p: {p}')
    print(f'q: {q}')

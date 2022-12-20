from itertools import cycle


if __name__ == '__main__':
    with open('Stage2.enc', 'rb') as f:
        data = f.read()

    key = cycle(b'th3_')

    decoded = bytes(map(lambda p: p[0] ^ p[1], zip(data, key)))

    with open('stage2.py', 'wb') as f:
        f.write(decoded)

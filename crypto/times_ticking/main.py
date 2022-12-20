from Crypto.Util.number import bytes_to_long, long_to_bytes
import base64
from random import randint, seed
from time import time
from datetime import datetime


def encrypt(text: str, code: int):
    return base64.b64encode(long_to_bytes(
        bytes_to_long(text.encode()) ^ code)).decode()


def decrypt(encrypted: str, code: int):
    decrypted = long_to_bytes(bytes_to_long(
        base64.b64decode(encrypted)) ^ code)

    try:
        decrypted = decrypted.decode()
    except UnicodeDecodeError:
        return None

    return decrypted


def get_otp(l: int):
    seed(round(time(), 5))
    return randint(10 ** l, 10 ** (l + 1) - 1)


def log(log: str):
    dt = datetime.now().strftime('%d/%m/%y %H:%M:%S')
    return f'[{dt}] {log}'


if __name__ == "__main__":
    choice = input(log('enter `e` to encrypt or `d` to decrypt: '))

    if choice == 'e':
        plain = input(log('enter plaintext: '))
        otp = get_otp(len(plain) * 2)
        e = encrypt(plain, otp)

        print(log(f'encrypted: {e}'))
        print(log(f'otp code: {otp}'))
    elif choice == 'd':
        encrypted = input(log('enter encrypted text: '))
        otp = int(input(log('enter otp code: ')))
        d = decrypt(encrypted, otp)

        print(f'decrypted message: {d}')

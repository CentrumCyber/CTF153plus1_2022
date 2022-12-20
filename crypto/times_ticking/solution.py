from Crypto.Util.number import bytes_to_long, long_to_bytes
import base64
from random import randint, seed
from datetime import datetime


def decrypt(encrypted, code):
    decrypted = long_to_bytes(bytes_to_long(
        base64.b64decode(encrypted)) ^ code)

    try:
        decrypted = decrypted.decode()
    except UnicodeDecodeError:
        return None

    return decrypted


def get_otp(l: int):
    return randint(10 ** l, 10 ** (l + 1) - 1)


ENCRYPTED = 'ZmzcWcD7FgLJRIq1IPo='
DATETIME = '28/09/22 08:14:20'
DECIMAL_PLACES = 5
FLAG_LENGTH = 14

t = datetime.timestamp(datetime.strptime(DATETIME, '%d/%m/%y %H:%M:%S'))
offset = 0

while True:
    _seed = (t * 10 ** DECIMAL_PLACES + offset) / 10 ** DECIMAL_PLACES
    seed(_seed)
    otp = get_otp(FLAG_LENGTH * 2)
    d = decrypt(ENCRYPTED, otp)

    if d and d.startswith('flag'):
        print(d)
        break

    offset += 1

# Time's ticking
## Challenge
```
[28/09/22 08:14:20] encrypted: ZmzcWcD7FgLJRIq1IPo=
```
and the main.py file.

## Solution
Plaintext is encrypted using OTP code which is randomly generated with seed of current time:
```py
def get_otp(l: int):
    seed(round(time(), 5))
    return randint(10 ** l, 10 ** (l + 1) - 1)
```
We can notice the line of output contains date and time which is provided by logging function. Knowing the exact second of encryption generation and knowing that seed is in this format:
```
>>> from time import time
>>> round(time(), 5)
1664346194.16354
>>>
```
we can try to guess the seed. Last thing we need to know is the length of plaintext (flag):
```
>>> import base64
>>> len(base64.b64decode('ZmzcWcD7FgLJRIq1IPo='))
14
>>>
```
Enter data and run a brute force:
```py
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
```
Output:
```
flag{153plus1}
```

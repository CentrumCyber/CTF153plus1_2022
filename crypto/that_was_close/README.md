# That was close...
## Solution
As we can see, public RSA key and some encrypted (probably with the public key) text are given. RSA keys are vulnerable for many types of attacks, especially in case where the key pair is badly generated. [Here](https://www.sjoerdlangkemper.nl/2019/06/19/attacking-rsa/) is a good article about RSA attacking methods.

After trying few of the most popular methods of cracking public RSA key we can figure out that [Fermat's factorization method](https://en.wikipedia.org/wiki/Fermat%27s_factorization_method) works for this case, because `p` and `q` are purposely generated in a way that they are close together. This is example python implementation of this attack which gives us `p` and `q` (private key) using only `n` (public key):
```python
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

```
Output:
```
p: 7696261743697318630479650620200915229638927479834674752245551806139659119290029435833788388297531231708418556841468563758072255232674053309012193370063
q: 7696261743697318630479650620200915229638927479834674752245551806139659118595434595481718897488775123031888522954381714824813674535834834994486812198223
```
Now when we have the private key (and `e` which is given as well) we can calculate inverse modulo of the private key and decrypt the encrypted text. But to make it even easier we can use [RsaCtfTool](https://github.com/RsaCtfTool/RsaCtfTool):
```
python RsaCtfTool.py -p 7696261743697318630479650620200915229638927479834674752245551806139659119290029435833788388297531231708418556841468563758072255232674053309012193370063 -q 7696261743697318630479650620200915229638927479834674752245551806139659118595434595481718897488775123031888522954381714824813674535834834994486812198223 -e 65537 --uncipher 51242120281704039402162996663592936358717512960053144797991397842727979055994827942385953846261866675383697589489062972544611932795160803698653631255654822710606629449010675763370846525276502475703372541020890941264317957500154251786268825372415512681099635921842384129680053102338137480662326171627986
# arguments:
python RsaCtfTool.py -p <p> -q <q> -e <e> --uncipher <encrypted_text>
```
Output:
```
Unciphered data :
HEX : 0x666c61677b313533706c7573317d
INT (big endian) : 2077392566271081823865090757308797
INT (little endian) : 2539219113152193410526059792788582
utf-8 : flag{153plus1}
utf-16 : 汦条ㅻ㌵汰獵紱
STR : b'flag{153plus1}'
HEX : 0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000666c61677b313533706c7573317d
INT (big endian) : 2077392566271081823865090757308797
INT (little endian) : 1341455570833457735963517292171749871229434549933122176855676548716770731320697802185251212958624789432702953102817848392207036504082833622803909257210522867655273804072820611993073526494111364536106417707369027611157424358728359997690229362503392341590289039433253698296527717967442703636632532315799552
utf-8 : flag{153plus1}
utf-16 : 汦条ㅻ㌵汰獵紱
STR : b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00flag{153plus1}'
```
Of course we are interested in `utf-8` and that's the flag.
from hashlib import sha256
from string import printable

flag2 = ''

# stage2.py:23
for c in printable:
	for d in printable:
		if sha256((c + d).encode('utf-8')).hexdigest() == 'e4223ed20d7ea5740a326e2b268ca6db91d041cf5194f577e393a8ba3b85d8e9':
			flag2 = c + d
			break

with open('Stage3.enc', 'rb') as f:
	encrypted_header = f.read(16)

header = b'\x7fELF\x02\x01\x01' + b'\x00' * 9

# stage2.py:30-40
flag2 += bytes([a ^ b for a, b in zip(reversed(encrypted_header), header)]).decode('utf-8')

# stage2.py:17
flag2 += chr(95)

print(flag2)
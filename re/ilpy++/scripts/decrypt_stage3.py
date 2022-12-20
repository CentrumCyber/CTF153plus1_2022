from itertools import cycle


def windows(arg, n):
	return (arg[i:i+n] for i in range(0, len(arg), n))


with open('Stage3.enc', 'rb') as f:
	encrypted_stage3 = f.read()

decrypted = b''

for stage3_chunk, key in zip(windows(encrypted_stage3, 16), cycle([b'r3_L4ngu4ges_th3'])):
	decrypted += bytes(map(lambda p: p[0] ^ p[1], zip(reversed(stage3_chunk), key)))

with open('stage3', 'wb') as f:
	f.write(decrypted)
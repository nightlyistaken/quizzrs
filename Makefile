run:
	clear
	DIFFICULTY=easy TTS=true cargo run
release:
	clear
	cargo build --release
test:
	clear
	DIFFICULTY=easy TTS=false cargo run
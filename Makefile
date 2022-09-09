run:
	diesel setup
	diesel migration run
	cargo run --bin write_post
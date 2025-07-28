PHONY: .release

release:
	cargo build --release && cp target/release/weather ~/.config/waybar/scripts/
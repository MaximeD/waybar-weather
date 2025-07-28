PHONY: .lint .release

# Format code source
lint:
	cargo fmt

# Create binary and move it to waybar’s config
release:
	cargo build --release && cp target/release/weather ~/.config/waybar/scripts/
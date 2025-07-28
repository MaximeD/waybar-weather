# waybar/weather

## Usage

Clone the repository and run `make release`.

Then in waybar configuration’s file add:
```json
# ~/.config/waybar/config

{
  "modules-<placement>": [
    "custom/weather"
  ],
  "custom/weather": {
    "format": "{}",
    "tooltip": true,
    "interval": 1800,
    "exec": "$HOME/.config/waybar/scripts/weather 'City, Country'",
    "return-type": "json"
  }
}
```

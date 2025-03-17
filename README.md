# Rust doorbell

This is a _very_ simple project that listens on MQTT and plays a pre-configured file. 
It is used to implement a zigbee powered door bell.

The project does not come with any sound files for licensing reasons, but you can find some here
https://pixabay.com/sound-effects/search/door-bell/

## License
MIT

## Usage 

```
Usage: mqtt-bell --config <CONFIG>

Options:
  -c, --config <CONFIG>  Path to config.toml
  -h, --help             Print help
  -V, --version          Print version
```


## Config 

```toml
[mqtt]
topic = "bell/ring"
host = "mqtt"

# Optional arguments
# port = 1833
# username = ""
# password = ""

[audio]
# Sound file must be specified, see https://crates.io/crates/rodio for supported formats
sound_file = ""
play_count = 3

[general]
log_level = "error"

```

## Additional dependencies
* ALSA, on systems using pipewire this means pipewire-alsa

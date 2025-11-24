# weatherframe

A service for querying OpenWeather for current weather data in a configurable location, rendering it into a nice image, and then displaying that image on an e-ink display. 

Almost all the complexity of this service is implemented in two other crates I cooked up for this project:
* [`openwx`](https://github.com/treyfortmuller/openwx): is a simple wrapper around the OpenWeather HTTP API
* [`tatted`](https://github.com/treyfortmuller/tatted): is a not-so-simple userspace e-ink controller driver for the JD79668 used in the 4.2" 4-color Pimoroni Inky wHAT display. It includes full support for color quantization and dithering of images with an arbitrary colorspace.

This service will be deployed as a systemd unit on a Raspberry Pi 4 running NixOS, the configuration for which can be found in my [pi-nixos](https://github.com/treyfortmuller/pi-nixos) repo.

```
nix develop

cargo run -- --help
```

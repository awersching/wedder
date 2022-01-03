<p align="center">
    <img src="img/conditions.gif"></img>
</p>

# wedder

<a href="https://github.com/awersching/wedder/actions?query=workflow%3ARelease"><img src="https://github.com/awersching/wedder/actions/workflows/release.yml/badge.svg" alt="Release Status"></a>
<a href="https://github.com/awersching/wedder/actions?query=workflow%3AMake"><img src="https://github.com/awersching/wedder/actions/workflows/make.yml/badge.svg" alt="Build Status"></a>
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Wedder displays the current weather condition and temperature in a configurable format for status bars like [polybar](https://github.com/jaagr/polybar).
The current weather is pulled from [OpenWeatherMap](https://openweathermap.org/) and you can either set a location manually or let it be determined by your IP.

## Installation

### AUR package

Install the [stable](https://aur.archlinux.org/packages/wedder/) or [development](https://aur.archlinux.org/packages/wedder-git/) AUR package with your package manager of choice.

### Download binary

Download a binary from the [releases page](https://github.com/awersching/wedder/releases).

### Compile from source

Compile the source code:

> `make build`

and place the executable in your path.

Note: [Cargo](https://github.com/rust-lang/cargo/) is required for building.

## Configuration

Because the OpenWeatherMap API is restricted, you have to get an API key by [creating a free account](https://home.openweathermap.org/users/sign_up).
After the API key is activated you can then pass it to wedder using

```bash
wedder -k <weather_api_key>
```

For more info use

```bash
wedder --help
```

You can also provide the API key via config file.
An example config is located [here](examples/wedder.toml).
The default path wedder expects for the config file can be viewed using

```bash
wedder -p
```

All config options except the weather condition icons can be overridden by passing CLI options.
The weather condition icons can only be set via config file.

Yet another option is providing the API key via an environment variable:

```bash
export WEDDER_WEATHER_API_KEY=<api_key>
```

### Polybar

Add a custom script module to your polybar config as shown in the [example snippet](examples/polybar).

### Fonts

The example config uses [Nerd Fonts](https://github.com/ryanoasis/nerd-fonts) to display the weather icons.
You can install the complete Nerd Fonts package but I wouldn't recommend it, as it is multiple GBs in size.
A better option is to pick a single font like [Noto Sans](https://github.com/ryanoasis/nerd-fonts/blob/master/patched-fonts/Noto/Sans/complete/Noto%20Sans%20Regular%20Nerd%20Font%20Complete.ttf) and install it manually.
For Linux this means placing the `.ttf` file in `/usr/share/fonts/TTF`.
There is also an [AUR package](https://aur.archlinux.org/packages/nerd-fonts-noto-sans-regular-complete/) for this single font.

Another option is using the [Weather Icons font](https://github.com/erikflowers/weather-icons) which is also included in Nerd Fonts.
If you are using other fonts however like [Font Awesome](https://github.com/FortAwesome/Font-Awesome) there will most likely be conflicts.

## Troubleshooting

### libssl error

When starting wedder, the following error occurs: 
```
wedder: error while loading shared libraries: libssl.so.1.0.0: cannot open shared object file: No such file or directory
```
Solution: install ```libssl1.0.0``` or ```openssl-1.0``` respectively according to your distribution.
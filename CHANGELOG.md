## Unreleased

- Add &lt;temperature_feels_like&gt; tag

## 1.1.1 - 2020-06-15

- Add troubleshooting to readme
- Add debug output formatting
- Fix negative coordinates as arguments
- Fix units not optional in config file

## 1.1.0 - 2020-05-05

- BREAKING: Add temperature unit and wind speed unit as separate config options (i.e. format changes from e.g. &lt;temperature_celsius&gt; to &lt;temperature&gt;)

## 1.0.6 - 2019-12-09

- BREAKING: No more default value for interval. If no interval is specified, wedder exits after printing the weather once

## 1.0.5 - 2019-04-11

- Add more weather info options
- Improve logging

## 1.0.4 - 2019-02-18

- Add binary release

## 1.0.3 - 2019-02-06

- Improve error handling
- Improve CLI arguments

## 1.0.2 - 2019-01-21

- BREAKING: rename icon names in config to snake case (e.g. ClearSky &rarr; clear_sky)
- Improve weather parsing
- Fix some icons having different sizes
- Add polybar config to open weather forecast in browser on module click
- Add option to print current city
- Improve debug output

## 1.0.1 - 2019-01-14

- Add config defaults (&rarr; config file is optional)
- Add cli options
- Add `exec-if` to polybar example
- Add better readme with gif

## 1.0.0 - 2019-01-12

- Initial release

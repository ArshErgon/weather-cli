# Weather CLI tool
> Made with the help weather api, which tells the temp in (max, min, normal) and about the information about sun rise and sun set.
It also use some other crates like progressbar, and colorText.

[weather_cli_ss.webm](https://user-images.githubusercontent.com/40994679/214584675-686a54c8-30cb-4454-8b77-4724e86480b8.webm)

# Running it locally

Make sure you have `rustc` or `cargo` installed in your machine.

fork the project and follow these steps
```git 
$ get clone https://www.github.com/YOUR_USERNAME/weather-cli
$ cd weather-cli && cargo run
````

# Docker

```git
$ docker build -t myapp .
$ docker run -p 8000:8000 myapp
```

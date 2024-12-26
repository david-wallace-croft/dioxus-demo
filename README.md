# Dioxus Demo

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/dioxus-demo/blob/main/LICENSE.txt

- Demonstration of Dioxus version 0.6

## Setup

- Install the Dioxus Command Line Interface (CLI)
```
cargo install dioxus-cli
```

## Usage

- To run it in your localhost browser
```
cd dioxus-demo/

dx serve --open
```

- To deploy using static site generation (SSG)
```
cd dioxus-demo/

rm -rf dist/

rm -rf static/

rm -rf target/dx/

dx build --release --ssg

mkdir dist/

cp -r target/dx/dioxus-demo/release/web/public/* dist/

cp -r static/* dist/

cd dist/

http-server -c-1 -o
```

## History

- Initial release: 2024-04-07

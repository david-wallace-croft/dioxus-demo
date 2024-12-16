# Dioxus Demo

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/dioxus-demo/blob/main/LICENSE.txt

- Demonstration of Dioxus version 0.6

## Usage

- To run it in your localhost browser
```
cd dioxus-demo/
cargo install dioxus-cli 
dx serve
```

- To deploy using static site generation (SSG)
```
cd dioxus-demo/
cargo install dioxus-cli 
rm -rf static/
rm -rf target/dx/
dx build --release --ssg
cp -r static/* target/dx/dioxus-demo/release/web/public/
```

## History

- Initial release: 2024-04-07

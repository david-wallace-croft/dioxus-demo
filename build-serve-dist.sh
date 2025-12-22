#!/bin/bash

rm -rf dist/

# Static site generation (SSG) fails if target/ is not clean
rm -rf target/

# It might be just this that is required instead of all of target/
# rm -rf target/server-release/

dx build --release --ssg --web

mkdir dist/

cp -r target/dx/dioxus-demo/release/web/public/* dist/

cd dist/

http-server -c-1 -o

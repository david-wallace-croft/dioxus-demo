#!/bin/bash

rm -rf dist/

rm -rf target/

dx build --release --ssg --web

mkdir dist/

cp -r target/dx/dioxus-demo/release/web/public/* dist/

cd dist/

http-server -c-1 -o

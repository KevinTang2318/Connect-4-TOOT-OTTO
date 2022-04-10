#!/bin/sh

# remove the previous files in the target folder

rm -rf target

# build the front end source codes
cd frontend && trunk build

cd ..

# copy compiled source files to the deploy folder to be used by backend
mkdir target/deploy
cp -rf frontend/dist/* target/deploy

# run backend
cargo run -p backend

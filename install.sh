#!/bin/sh

# git clone
git clone https://github.com/atsushi130/IgnoreGen.git
cd IgnoreGen

# build application
cargo build --release

# mkdir ~/.ignoregen/bin
mkdir ~/.ignoregen
mkdir ~/.ignoregen/bin

# copy built application
cp ./target/release/ignoregen ~/.ignoregen/bin/

# remove
cd ../
rm -rf ./IgnoreGen

# Quadrium
[![Rust Ubuntu](https://github.com/SIL3nCe/Quadrium/workflows/Rust/badge.svg?branch=master)](https://github.com/SIL3nCe/Quadrium/actions/workflows/rust-ubuntu.yml)

An audio player written in Rust

# Status
Currently in development and so will not work as expected. It is only in early development and so **does not** offer :
* Play FLAC music files or other music files
* Well defined and feature complete User Interface

# Install
To install this project, you need to :
* clone the repository with all the submodule by doing ```git clone --recursive https://github.com/SIL3nCe/Quadrium.git```
* build the project with cargo

Currently the project compiled on Windows and Linux. The other operating systems are not tested and so the support is not garanteed.

# Depedencies
* iced 0.10.0
* png 0.17.7

Currently, Symphonia is present in the cargo.toml but is not currently use.
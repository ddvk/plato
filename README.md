# Plato reader - Remarkable Tablet port fork
## Based on a very old version and libraries

![Logo](artworks/plato-logo.svg)

*Plato* is a document reader for *Kobo*'s e-readers.


## Supported formats

- PDF, ePUB and CBZ via *mupdf*.
- DJVU via *djvulibre*.


## How to Build (with the remarkable toolchain)

- install rust nightly (last tested with version 1.41)

   `rustup install nightly`

   `rustup default nightly`

- add the arm target

   `rustup target adduarmv7-unknown-linux-gnueabihf`

- install the remarkable toolchain from (remarkable.engineering)
- source the env

    `source /usr/local/oecore-x86_64/environment-setup-cortexa9hf-neon-oe-linux-gnueabi`

- run `build.sh fast` once, which should create the libs folder with all C libraries
- build
    
    `cargo build --target=armv7-unknown-linux-gnueabihf`

## TODO
- update the libraries and compile them with the toolchain

## Features

- Hierarchical categories.
- The metadata for each document is read from a single JSON file.
- Crop margins of non-reflowable documents.

[![Tn01](artworks/thumbnail01.png)](artworks/screenshot01.png) [![Tn02](artworks/thumbnail02.png)](artworks/screenshot02.png)

[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=KNAR2VKYRYUV6)

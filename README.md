# chain-registry

A Rust API for interacting with the [Cosmos chain registry repository](https://github.com/cosmos/chain-registry)

## Warning

The chain registry is unversioned and syntax is unenforced. This library is written to ignore unrecognized or missing JSON
fields but it isn't guaranteed to work for all registry items.

## Features

- Models for serializing and deserializing chain.json, assets.json and IBC path JSON files
- Simple get/list methods for retrieving chain, asset, and path data
- A cache type (currently only supports IBC Path data) that exposes additional filtering options

## To do

- Test which queries all objects
- Release per registry commit?

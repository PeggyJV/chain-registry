# chain-registry

A Rust API for interacting with the [Cosmos chain registry repository](https://github.com/cosmos/chain-registry)

## Features

- Models for serializing and deserializing chain.json, assets.json and IBC path JSON files
- Simple get/list methods for retrieving chain, asset, and path data
- A cache type (currently only supports IBC Path data) that exposes additional filtering options

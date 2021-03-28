#!/bin/bash

# Install rkeep to .cargo/bin
cargo install --path .

# Copy sample and service
mkdir ~/.config/rkeep
cp config.sample.toml ~/.config/rkeep/
cp rkeepd.service ~/.config/systemd/user/

#!/bin/bash

sudo apt update

sudo apt install -y lld pkg-config libssl-dev build-essential

cargo install sqlx-cli

export DATABASE_URL="postgres://docker:docker@localhost:5432/verdantvibes?search_path=backend"
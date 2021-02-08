#!/bin/bash

# Exit on error
set -euxo pipefail

# Build frontend
cd frontend
quasar clean
quasar build

# Build backend
cargo build --release

#!/bin/bash
# Extract dependencies from Cargo.toml and run cargo whatfeatures on each

# Using grep and sed to extract dependency names from Cargo.toml
deps=$(grep -E "^[a-z_-]+ *=|^[a-z_-]+ *{" Cargo.toml | sed -E 's/^([a-z_-]+).*/\1/')

# Run cargo whatfeatures for each dependency
for dep in $deps; do
  echo "======== Features for $dep ========"
  cargo whatfeatures $dep
  echo ""
done


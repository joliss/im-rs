#!/usr/bin/env bash

PROJECT_ROOT=`git rev-parse --show-toplevel`
PREVIOUS_CWD=`pwd`

cd $PROJECT_ROOT

# get version from im-core
VERSION=`sed -n 's/^version *= *"\(.*\)"/\1/p' im-rc/Cargo.toml`

# bump version
VERSION=`./semver.sh $1 $VERSION`

echo "Version => $VERSION"

# update version in im-core/Cargo.toml
sed -i -E "s/^version = \".*\"/version = \"$VERSION\"/" im-rc/Cargo.toml

# set versions in im/Cargo.toml
sed -i -E "s/^version = \".*\"/version = \"$VERSION\"/" im/Cargo.toml
sed -i -E "s/^im-core = \".*\"/im-core = \"$VERSION\"/" im/Cargo.toml
sed -i -E "s/^im-arc = \".*\"/im-arc = \"$VERSION\"/" im/Cargo.toml

# generate im-arc
cp im-arc/README.md /tmp/im-arc-README.md
rm -rf im-arc
cp -a im-rc im-arc
mv /tmp/im-arc-README.md im-arc/README.md
sed -i -E 's/^name = "im-rc"/name = "im-arc"/' im-arc/Cargo.toml
sed -i -E "s/single threaded/thread safe/" im-arc/Cargo.toml
sed -i -E "s/^version = \".*\"/version = \"$VERSION\"/" im-arc/Cargo.toml

# build top level
cargo build --package im

# reset env
cd $PREVIOUS_CWD

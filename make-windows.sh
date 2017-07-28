#/bin/bash
set -e

name=playground
if [ -n "$1" ]; then
    name=$1
fi

cargo_target=x86_64-pc-windows-gnu
cargo build --release --example $name --target $cargo_target

target_dir=target/web/windows/$name
mkdir -p $target_dir
cp target/$cargo_target/release/examples/$name.exe $target_dir
cp -r examples/$name/static/assets $target_dir

pushd $target_dir
rm -f ../$name.zip
zip -r ../$name.zip .
popd
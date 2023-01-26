# superglue
Tool for write a small file to the beginning of a large file in place.
This can be useful for gluing runtime and squashfs image or shell script and archive.

## To get started:
* **Download the latest revision**
```
git clone https://github.com/VHSgunzo/superglue.git && cd superglue
```
* **Usage**
```
./superglue.py {small file} {large file}
```
* Or for gluing fully in memory:
```
./bufcat.py {small file} {large file}
```

* Rust version:
```
./superglue {small file} {large file}
```

* **You can also compile a binary Rust version**
```
rustup default nightly
rustup target add x86_64-unknown-linux-musl
rustup component add rust-src --toolchain nightly
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --release
```
* Or take an already precompiled binary file from the [releases](https://github.com/VHSgunzo/superglue/releases)

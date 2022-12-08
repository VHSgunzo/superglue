# superglue
Superglue for gluing runtime and squashfs image or shell script and archive

## To get started:
* **Download the latest revision**
```
git clone https://github.com/VHSgunzo/superglue.git && cd superglue
```
* **Usage**
```
./superglue.py {runtime|script} {squashfs|archive}
```
* Or for gluing fully in memory
```
./bufcat.py {runtime|script} {squashfs|archive}
```

* **You can also compile a binary Rust version**
```
cargo build --release 
```
* Or take an already precompiled binary file from the [releases](https://github.com/VHSgunzo/superglue/releases)
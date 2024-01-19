# arroy-edge

## Build
You should install `lmdb` first
On windwos, use `msys2` with `ucrt64`
```console
pacman --needed -S $MINGW_PACKAGE_PREFIX-{lmdb,rust}
git clone https://github.com/thewh1teagle/arroy-edge
cd arroy-edge
cargo run
```

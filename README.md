# arroy-edge

## Build
You should install `lmdb` first
On windwos, use `msys2` with `ucrt64`
```console
pacman --needed -S $MINGW_PACKAGE_PREFIX-{lmdb,rust}
git clone https://github.com/thewh1teagle/arroy-edge
cd arroy-edge
cargo build --release
```

## Using
1. Grab OpenAI api key from [openai.com/api-keys](https://platform.openai.com/api-keys)
2. Copy `.example.env` to `.env` and paste the key
3. Execute the program, add some inputs and search


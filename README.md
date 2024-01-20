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

## Example
```console
$ ./arroy-edge add "I like banana"
✅ Saved 2680824772

$ ./arroy-edge add "The most loved animal in the world is the dog."
✅ Saved 673299976

$ ./arroy-edge search "Do you know some loved animal?"
✅ Found: The most loved animal in the world is the dog.

$ ./arroy-edge search "What do you like?"
✅ Found: I like banana
```

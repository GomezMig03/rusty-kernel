# Rusty kernel
Personal project made to learn rust and operative systems at the same time.

## To develop locally
You'll need to have git, rust, cargo and qemu installed on your system, then you can clone the repo with the following commands:
```
git clone https://github.com/GomezMig03/rusty-kernel.git
cd rusty-kernel/
```
Then you can test the kernel, this command will compile and invisibly run qemu to run the tests:
```
cargo test
```

Or directly run it, this will compile the kernel, make it bootable thanks to the bootloader and bootimage dependencies, and then run it through qemu:
```
cargo run
```
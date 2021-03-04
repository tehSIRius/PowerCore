# PowerCore

PowerCore is a moonshot idea of a nice, small, and modular operating system written in Rust. It currently supports x86 architecture with a dream of running on ARM.

## How to Run

* Run `cargo install` and `cargo build` to make sure everything works and all dependencies are present.
* You can get a live version in QEMU by running `cargo run`
* After running `cargo bootimage`, you can use the raw binary from `target/power_core_x86/debug/bootimage-power_core.bin`
  * Through QEMU with `qemu-system-x86_64 -drive format=raw,file=target/target/power_core_x86/debug/bootimage-power_core.bin`
  * Through USB boot image for a real machine with `dd if=target/x86_64-blog_os/debug/bootimage-blog_os.bin of=/dev/sdX && sync`
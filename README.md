#### Write OS onto a USB drive

- `sdX` is the device name of your USB stick.
- Doesn’t work for UEFI machines since the bootloader crate has no UEFI support yet

> **WARNING :** This will overwrite the contents of the USB drive

```bash
dd if=target/x86_64-blog_os/debug/bootimage-blog_os.bin of=/dev/sdX && sync
```

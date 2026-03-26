# Rayon-Ca.BYOND

Is a crate that integrates basics for interaction with rayon-ca for BYOND projects.

Rayon-Ca: https://github.com/AlexMorgan3817/Rayon-Ca.Byond.git

![alt text](.github/image.png)

## Building

For release purpose:
```rs
cargo bld
```
It will provide prompt of your target system.

Or you can build it directly through macroses:
```rs
cargo br  // For Linux arch
cargo brw // For Window arch
```

### Debuging
```rs
cargo dbg
```
compiles lib and sends it to test/ folder for use by test/test.dmb

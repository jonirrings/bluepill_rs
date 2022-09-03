## Usage
**Build the project**
```shell
    cargo build --release
```
**Flash to target**
```shell
    cargo flash --chip STM32F103C8 --connect-under-reset --release
````
**If updating linker script or things Cargo might not notice, it can be helpful to follow up with**
```shell
    cargo clean
```
**strip**
```shell
   rust-objcopy --binary-architecture=thumbv7m bluepill_rs --strip-all -O binary bluepill.bin
```

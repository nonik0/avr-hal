# Flashing

```sh
$ cargo build --bin feather-blink
$ avrdude -v -p m32u4 -cavr109 -P /dev/ttyX -D -Uflash:w:../../target/avr-atmega32u4/debug/feather-blink.elf:e
```

```cmd
> cargo build --bin feather-blink
> avrdude -v -p m32u4 -cavr109 -P COMX -D -Uflash:w:../../target/avr-atmega32u4/debug/feather-blink.elf:e
```
set tdesc filename target.xml
set architecture i8086
set tdesc filename target.xml
b *0x7c00
target remote 127.0.0.1:1234
continue

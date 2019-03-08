#!/bin/bash
objdump -b binary -D boot_sector.bin -m i8086 --adjust-vma=0x7c00
exit 0

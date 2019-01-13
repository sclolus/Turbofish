arch := i686
kernel := build/kernel.elf
name := undefined_behavior
cargo_flags := 

# https://linux.die.net/man/1/ld
linker_params := -e "_start" --oformat=elf32-i386 -Ttext-segment=0x100000

assembly_source_files := $(wildcard src/asm/*.asm)
assembly_object_files := $(patsubst src/asm/%.asm, \
    build/asm/%.o, $(assembly_source_files))

target := $(arch)-unknown-linux-gnu

ifeq ($(DEBUG),yes)
rust_os := target/$(target)/debug/lib$(name).a
else
cargo_flags += --release
rust_os := target/$(target)/release/lib$(name).a
endif

.PHONY: all clean kernel

all: $(kernel)

fclean:
	rm -rf build
	cargo clean

clean: fclean

kernel: $(kernel)

$(kernel): build_rust $(rust_os) $(assembly_object_files) $(linker_script)
	ld -m elf_i386 -n --gc-sections $(linker_params) -o $(kernel) $(assembly_object_files) $(rust_os)

build_rust:
	cargo build $(cargo_flags) --target $(target)

# compile assembly files
build/asm/%.o: src/asm/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf $< -o $@

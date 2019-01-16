arch := x86_64
kernel := build/kernel.elf
name := undefined_behavior
cargo_flags := 

linker_script := src/linker.ld

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
	ld -m elf_x86_64 -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

build_rust:
	cargo rustc $(cargo_flags) --target $(target)

# compile assembly files
build/asm/%.o: src/asm/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@

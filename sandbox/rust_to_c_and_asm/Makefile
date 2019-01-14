arch := x86_64
kernel := build/multilang_program
name := rust_to_c_and_asm
cargo_flags :=

# https://linux.die.net/man/1/ld
linker_params := -e "main"

assembly_source_files := $(wildcard src/asm/*.asm)
assembly_object_files := $(patsubst src/asm/%.asm, \
    build/asm/%.o, $(assembly_source_files))

c_source_files := $(wildcard src/c/*.c)
c_object_files := $(patsubst src/c/%.c, \
    build/c/%.o, $(c_source_files))

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

$(kernel): build_rust $(rust_os) $(assembly_object_files) $(c_object_files)
	#ld $(linker_params) -o $(kernel) $(assembly_object_files) $(c_object_files) $(rust_os)
	gcc -o $(kernel) $(assembly_object_files) $(c_object_files) $(rust_os)

build_rust:
	cargo build $(cargo_flags) --target $(target)

# compile assembly files
build/asm/%.o: src/asm/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf $< -o $@

# compile c files
build/c/%.o: src/c/%.c
	mkdir -p $(shell dirname $@)
	gcc -m64 -c -g -O0  $< -o $@

arch := ivybridge_cpu
kernel := build/kernel.elf
name := undefined_behavior
# These rustc flags are defined in json xbuild file.
rustc_flags := -C no-redzone -C target-feature=-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2

linker_script := src/linker.ld

assembly_source_files := $(wildcard src/asm/*.asm)
assembly_object_files := $(patsubst src/asm/%.asm, \
    build/asm/%.o, $(assembly_source_files))

target := $(arch)
custom_target := $(arch).json
#$ come from this output: rustc -Z unstable-options --print target-spec-json --target i686-unknown-linux-gnu
#doc about this flags: https://github.com/rust-lang/rust/blob/256e497fe63bf4b13f7c0b58fa17360ca849c54d/src/librustc_back/target/mod.rs#L228-L409 

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
	ld -m elf_i386 -n --gc-sections -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

build_rust:
	cargo xbuild $(cargo_flags) --target $(custom_target) --verbose

# compile assembly files
build/asm/%.o: src/asm/%.asm
	mkdir -p $(shell dirname $@)
	nasm -felf $< -o $@

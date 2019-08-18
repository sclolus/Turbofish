all: src/libc.rs

src/libc.rs: all_includes.h ../../programs/libc/include/* Makefile
	@./bindgen.sh all_includes.h > src/libc.rs
	@# echo $?

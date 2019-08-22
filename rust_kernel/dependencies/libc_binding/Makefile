all: updated

updated: all_includes.h ../../programs/libc/include/* Makefile
	@echo updated > updated
	@./bindgen.sh all_includes.h > src/libc.rs
	@# echo $?

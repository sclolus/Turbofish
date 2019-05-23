NAME = build/bootstrap.o

CC = gcc
ASM = nasm
LD = ld

LDSCRIPT = linker.ld
LDFLAGS = -m elf_i386
ifeq ($(LDMAP),yes)
	LDFLAGS += -M
endif
LDFLAGS += -r

ASMFLAGS = -f elf

### MAIN FLAGS ###

ifeq ($(DEBUG),yes)
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -g -O0
else ifeq ($(OPTIM),yes)
	# -fno-omit-frame-pointer keep good call trace for panic()
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall -O3 -fno-omit-frame-pointer
else
	CFLAGS = -m32 -std=gnu99 -Wextra -Wall
endif
CFLAGS += -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions -nostdinc -fno-pie

IFLAGS = -Iinclude -Ilibft/includes -I./

SRC_C += get_device_mem_map vga_text early_gdt early_panic early_idt early_watchdog
SRC_ASM += 8086_payload payload_get_mem_map init_paging idt_cpu_handler
VPATH += src/
HEADERS += vga_text.h early_gdt.h tss.h early_idt.h early_panic.h early_paging.asm early_watchdog.h

OBJ_DIR := objs
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)
	objcopy --prefix-symbols=alt_ $@

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS)
	$(ASM) $(ASMFLAGS) -o $@ $<
	objcopy --prefix-symbols=alt_ $@

.PHONY: all clean fclean common_clear re help $(NAME)

ifeq ($(OWNED_BY_KERNEL),yes)
all: build_libft $(NAME)
else
# *** for auto-compilation of the library ***
all: build_libft autobuild/nm_map_gen $(NAME)
# *** *** *** *** *** *** *** *** *** *** ***
endif

LIBS = libft/alt_libft.a

$(NAME): $(OBJ_C) $(OBJ_ASM) $(LIBS)
	$(compile_early_panic)
	$(link)

compile_early_panic = $(CC) -c $(CFLAGS) -o $(OBJ_DIR)/early_panic.o src/early_panic.c $(IFLAGS) && objcopy --prefix-symbols=alt_ $(OBJ_DIR)/early_panic.o

# *** for auto-compilation of the library ***
extract_kernel_symbols = nm -n $@ > autobuild/nm_output
gen_symbol_map = autobuild/nm_map_gen autobuild/nm.map autobuild/nm_output && sync
# *** *** *** *** *** *** *** *** *** *** ***

link = $(LD) $(LDFLAGS) $^ -o $@

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $< $(IFLAGS)
	objcopy --prefix-symbols=alt_ $@

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS)
	$(ASM) $(ASMFLAGS) -o $@ $<
	objcopy --prefix-symbols=alt_ $@

# *** for auto-compilation of the library ***
autobuild/nm_map_gen: autobuild/nm_map_gen.c
	gcc -Wextra -Wall $< -o $@ -g -O0 -fsanitize=address
	$(gen_symbol_map)
# *** *** *** *** *** *** *** *** *** *** ***

build_libft:
	make -C libft all DEBUG=$(DEBUG) OPTIM=$(OPTIM) ALT=yes

common_clean:
	find $(OBJ_DIR) -type f -name '*.o' -delete
	rm -f autobuild/nm_output
	rm -f autobuild/nm.map
	rm -f autobuild/nm_map_gen
	rm -f $(NAME)

clean: common_clean
	make -C libft/ clean

fclean: common_clean
	make -C libft/ fclean

re: fclean all

help:
	@echo
	@echo "object $(NAME)"
	@echo
	@echo "--------------------------------------------------------------------------"
	@echo " Disp rules."
	@echo
	@echo " all     : Compile the kernel $(NAME)."
	@echo " re      : Recompile all objets of the programs."
	@echo " clean   : Remove objects."
	@echo " fclean  : Remove objects and programs."
	@echo " help    : Display this."
	@echo "--------------------------------------------------------------------------"

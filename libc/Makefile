# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

TARGET_UNIT_TESTS= unit-tests
TARGET_LIBC= libc.a
ifeq ($(unit-tests),yes)
	LD_LIBRARY_PATH := /usr/local/lib:$(LD_LIBRARY_PATH)
	TARGET = $(TARGET_UNIT_TESTS)
	AS = as
	CC = gcc
	AR = ar
	RANLIB = ranlib
	CFLAGS += -D UNIT_TESTS -g3 -fsanitize=address
	ASFLAGS = --32
	IFLAGS = -Iinclude/ltrace -Iinclude/tools
else
	TARGET = $(TARGET_LIBC)
	AS = i686-turbofish-as
	CC = i686-turbofish-gcc
	AR = i686-turbofish-ar
	RANLIB = i686-turbofish-ranlib
	IFLAGS = -Iinclude -I./include/ltrace -Iinclude/tools
	ASFLAGS = --32
endif

# non-turbofish specific program. Use carefully
NASM = nasm

SRC_DIR = src
OBJ_DIR = obj
SRC_C = $(shell find $(SRC_DIR) -type f -iname "*.c")
SRC_ASM = $(shell find $(SRC_DIR) -type f -iname "*.s")
SRC_NASM = $(shell find $(SRC_DIR) -type f -iname "*.asm")
SRC_C_RUNTIME = $(SRC_DIR)/crt/crt0.s $(SRC_DIR)/crt/crti.s $(SRC_DIR)/crt/crtn.s $(SRC_DIR)/crt/crtbegin.s $(SRC_DIR)/crt/crtend.s

PRIVATE_HEADERS += user_syscall.h

TESTED_SRC_C = strsplit array getenv setenv unsetenv clearenv execl getmntent
HEADERS += string.h

HEADERS += stdio.h
PRIVATE_HEADERS += internal_printf.h

HEADERS += stdlib.h
PRIVATE_HEADERS += main_headers.h alloc_btree_internal_header.h

HEADERS += stdlib.h

HEADERS += unistd.h

HEADERS += sys/mman.h

HEADERS += sys/stat.h

VPATH += sys/select.h

HEADERS += sys/statfs.h

HEADERS += sys/statvfs.h

HEADERS += sys/time.h

HEADERS += sched.h

HEADERS += time.h

HEADERS += signal.h

HEADERS += wait.h sys/wait.h

HEADERS += sys/socket.h sys/un.h

HEADERS += locale.h

HEADERS += termios.h

HEADERS += dirent.h

HEADERS += fcntl.h

HEADERS += ctype.h

HEADERS += math.h

HEADERS += setjmp.h

HEADERS += math.h

HEADERS += tools/tools.h

HEADERS += pwd.h

HEADERS += utime.h

HEADERS += grp.h

HEADERS += strings.h

HEADERS += mntent.h

HEADERS += sys/types.h errno.h stdint.h sys/param.h inttypes.h limits.h assert.h custom.h
# bullshit headers needed by coreutils and dash
HEADERS += getopt.h sys/resource.h sys/ioctl.h sys/times.h libintl.h mntent.h

HEADERS += ltrace/ltrace.h
CFLAGS += -O2 -Wall -Wextra -fno-omit-frame-pointer -masm=intel $(IFLAGS)
ifeq ($(trace),yes)
	CFLAGS += -D LTRACE
endif

OBJ_C_RUNTIME = $(SRC_C_RUNTIME:$(SRC_DIR)/crt/%.s=$(OBJ_DIR)/crt/%.o)
OBJ_NASM = $(SRC_NASM:$(SRC_DIR)/%.asm=$(OBJ_DIR)/%.o)
OBJ_ASM = $(SRC_ASM:$(SRC_DIR)/%.s=$(OBJ_DIR)/%.o)
OBJ_C = $(SRC_C:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)

TESTED_OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(TESTED_SRC_C)))))
SYSROOT_HEADERS = $(addprefix $(TOOLCHAIN_SYSROOT)/usr/include/, $(HEADERS))
VPATH += include/:$(SRC_DIR)/stdio/printf:$(SRC_DIR)/stdlib/alloc:$(SRC_DIR)/stdlib/alloc/alloc_btree

.PHONY = all clean fclean re exec
all: $(TARGET) $(OBJ_C_RUNTIME) $(SYSROOT_HEADERS)

$(TOOLCHAIN_SYSROOT)/usr/include/%: include/%
	@mkdir -pv $(dir $@)
	@cp -v $< $@

$(TARGET_LIBC): $(OBJ_C_RUNTIME) $(OBJ_NASM) $(OBJ_ASM) $(OBJ_C)
	$(AR) rc $@ $^
	$(RANLIB) $@
	cp -v libc.a $(TOOLCHAIN_SYSROOT)/usr/lib/.
	cp -v $(OBJ_C_RUNTIME) $(TOOLCHAIN_SYSROOT)/usr/lib/.

$(TARGET_UNIT_TESTS): $(TESTED_OBJ_C)
	$(CC) $(CFLAGS) -o $@ $^ -lcriterion

$(OBJ_DIR)/crt/%.o: $(SRC_DIR)/crt/%.s
	@mkdir -pv $(@D)
	$(AS) $(ASFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.asm Makefile $(HEADERS) $(PRIVATE_HEADERS)
	@mkdir -pv $(@D)
	$(NASM) -f elf -o $@ $<

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c Makefile $(HEADERS) $(PRIVATE_HEADERS)
	@mkdir -pv $(@D)
	$(CC) -c $(CFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.s Makefile $(HEADERS) $(PRIVATE_HEADERS)
	@mkdir -pv $(@D)
	$(AS) $(ASFLAGS) -o $@ $<

clean:
	find $(OBJ_DIR) -type f -name '*.o' -exec rm -fv {} \;

fclean: clean
	rm -fv $(TARGET_LIBC)
	rm -fv $(TARGET_UNIT_TESTS)
	find $(OBJ_DIR) -mindepth 1 -delete

re: fclean all

exec: all
	./$(TARGET_LIBC)

install:
	cp -v libc.a $(TOOLCHAIN_SYSROOT)/usr/lib/.
	cp -v $(OBJ_C_RUNTIME) $(TOOLCHAIN_SYSROOT)/usr/lib/.
	cp -rv include/* $(TOOLCHAIN_SYSROOT)/usr/include/

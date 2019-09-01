PATH  := $(PATH):/toolchain_turbofish/cross/bin/
SHELL := env PATH=$(PATH) /bin/bash

BUILD_DIR = build
VPATH += src

AS = i686-turbofish-ld
CC = i686-turbofish-gcc

SRC_C += DeepThought \
		ProcessGroup \
		signal/SignalSimple \
		signal/SignalSimpleDuo \
		execve/argv \
		execve/check_argv \
		sigprocmask/sigprocmask \
		wait/wait \
		wait/wuntraced \
		munmap/munmap \
		mprotect/mprotect \
		mmap/mmap \
		isatty/isatty \
		atexit/atexit \
		pipe/pipe_fucker \
		pipe/pipe_fister \
		pipe/pipe_lorem_ipsum \
		math/roundf \
		ctype/longlong \
		lseek/sda \
		lseek/lseek_return \

VPATH += src/signal src/execve src/sigprocmask src/wait src/munmap src/mprotect src/mmap src/isatty src/atexit src/pipe src/math

OBJ_DIR = obj
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(SRC_C)))

IFLAGS = -I include

CFLAGS = -Wall -Wextra -fno-omit-frame-pointer
ifeq ($(GNU),yes)
CFLAGS += -m32 -D GNU
CC = gcc
LIB = -lm
else
endif


TARGET_C = $(addprefix $(BUILD_DIR)/, $(SRC_C))
INSTALL_DIR = ../../rust_kernel/src/userland/

.PHONY = all clean fclean re

all: $(TARGET_C)

build/%: $(OBJ_DIR)/%.o Makefile /toolchain_turbofish/sysroot/usr/lib/libc.a obj/tools.o
	@mkdir -pv ./$(dir $@)
	$(CC) $(CFLAGS) -o $@ $< obj/tools.o $(LIB)

$(OBJ_DIR)/%.o: %.c Makefile include/tools.h
	@mkdir -pv ./$(dir $@)
	$(CC) $(CFLAGS) $(IFLAGS) -c -o $@ $<

$(OBJ_C):

obj/tools.o:

install:
	cp -rv build/* $(INSTALL_DIR)

clean:
	rm -fv $(TARGET_C)
	find $(OBJ_DIR)/ -name "*.o" -delete

fclean: clean

re: fclean all

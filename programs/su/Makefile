PATH  := $(PATH):/toolchain_turbofish/cross/bin/
SHELL := env PATH=$(PATH) /bin/bash

ifeq ($(make_pass),yes)
	TARGET = hasher
	CFLAGS += -D MAKE_PASS
else
	TARGET = su
endif

ifeq ($(debug),yes)
	CFLAGS += -g3 -fsanitize=address
else
ifneq ($(unit-tests), yes)
	CFLAGS += -static
endif
endif

TARGET_SU=su
TARGET_UNIT_TESTS=unit-tests

ifeq ($(unit-tests),yes)
	LD_LIBRARY_PATH := /usr/local/lib:$(LD_LIBRARY_PATH)
	TARGET = $(TARGET_UNIT_TESTS)
	AS = as
	CC = gcc
	AR = ar
	RANLIB = ranlib
	CFLAGS += -D UNIT_TESTS -g3 -fsanitize=address
	ASFLAGS = --32
	IFLAGS =
else
	TARGET = $(TARGET_SU)
	AS = i686-turbofish-as
	CC = i686-turbofish-gcc
	AR = i686-turbofish-ar
	RANLIB = i686-turbofish-ranlib
	IFLAGS = -Iinclude
endif



SRC_C += main \
	strsplit \
	get_file_contents \
	md5 \
	encode_base64.c \
	decode_base64.c \
	hash_fuzzer \
	hash_tester \
	left_rotate_32 \
	print_hash \
	print_memory \
	passwd \
	shadow \
	utils \
	getopt \
	getpass

HEADERS += include/su.h

VPATH += src

IFLAGS = -I include

TEST=  -DTESTS
CFLAGS += -O2 -Wall -Wextra -fno-omit-frame-pointer $(IFLAGS)

.PHONY = all clean fclean re exec

TEST_DIR = tests

OBJ_DIR = objs
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))
INSTALL_DIR = ../../rust_kernel/src/userland/

all: $(TARGET) Makefile

$(TARGET): $(OBJ_C)
	$(CC) -o $@ $(CFLAGS) $^
	# sudo chown root:root $(TARGET)
	# sudo chmod u+s $(TARGET)

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	@mkdir -pv $(OBJ_DIR)
	$(CC) -c $(CFLAGS) -o $@ $<

install:
	cp -rv $(TARGET) $(INSTALL_DIR)

clean:
	find $(OBJ_DIR) -type f -name '*.o' -exec rm -fv {} \;

fclean: clean
	rm -fv $(TARGET)

re: fclean all

test:
	bash test.sh

unit-tests: $(OBJ_C)
	$(CC) -o unit-tests $(CFLAGS) $^ -lcriterion
	./unit-tests

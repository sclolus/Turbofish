# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

BUILD_DIR = bin
VPATH += src

# AS = i686-turbofish-ld
CC = i686-turbofish-gcc

SRC_C += init

CFLAGS = -Wall -Wextra -fno-omit-frame-pointer -I include
ifeq ($(GNU),yes)
CFLAGS += -m32 -D GNU
CC = gcc
else
endif

#ASFLAGS = -f elf

ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

TARGET_C = $(addprefix $(BUILD_DIR)/, $(SRC_C))

.PHONY = all clean fclean re

all: $(TARGET_C)

$(BUILD_DIR)/%: %.c $(LIBC_AR) Makefile
	$(CC) $(CFLAGS) -o $@ $<
	cp -rv --parents $@ $(SYSTEM_ROOT)

clean:
	rm -fv $(TARGET_C)

fclean: clean

re: fclean all

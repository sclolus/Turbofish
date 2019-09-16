# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

BUILD_DIR = bin
VPATH += src

AS = i686-turbofish-ld
CC = i686-turbofish-gcc

SRC_C += init \

VPATH += src

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

.PHONY = all clean fclean re

all: $(TARGET_C)

$(BUILD_DIR)/%: $(OBJ_DIR)/%.o Makefile $(LIBC_AR)
	@mkdir -pv ./$(dir $@)
	$(CC) $(CFLAGS) -o $@ $< $(LIB)
ifneq ($(GNU),yes)
	cp -rv --parents $@ $(SYSTEM_ROOT)
endif

$(OBJ_DIR)/%.o: %.c Makefile
	@mkdir -pv ./$(dir $@)
	$(CC) $(CFLAGS) $(IFLAGS) -c -o $@ $<

$(OBJ_C):

clean:
	rm -fv $(TARGET_C)
	find $(OBJ_DIR)/ -name "*.o" -delete

fclean: clean

re: fclean all

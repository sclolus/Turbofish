# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

TARGET_UNIT_TESTS = unit-tests
TARGET_LOGIN = login

ifeq ($(debug),yes)
	CFLAGS += -g3 -fsanitize=address
else
ifneq ($(unit-tests), yes)
	CFLAGS += -static
endif
endif


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
	TARGET = $(TARGET_LOGIN)
	AS = i686-turbofish-as
	CC = i686-turbofish-gcc
	AR = i686-turbofish-ar
	RANLIB = i686-turbofish-ranlib
	IFLAGS = # -Iinclude
endif

SRC_C += login
HEADERS +=

VPATH += src

IFLAGS = # -I include

TEST=  -DTESTS
CFLAGS += -O2 -Wall -Wextra -fno-omit-frame-pointer $(IFLAGS)

.PHONY = all clean fclean re exec

OBJ_DIR = objs
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

BUILD_DIR = bin

all: $(TARGET) Makefile

$(TARGET): $(OBJ_C)
	make -C ../su 		# well.
	@mkdir -pv $(BUILD_DIR)
	$(CC) -o $(BUILD_DIR)/$@ $(CFLAGS) $^
	sudo chown root:root $(BUILD_DIR)/$(TARGET)
	cp -v --parents $(BUILD_DIR)/$@ $(SYSTEM_ROOT)

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	@mkdir -pv $(OBJ_DIR)
	$(CC) -c $(CFLAGS) -o $@ $<

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

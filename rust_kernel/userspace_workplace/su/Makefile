ifeq ($(make_pass),yes)
	TARGET = hasher
	CFLAGS += -D MAKE_PASS
else
	TARGET = su
endif

ifeq ($(debug),yes)
	CFLAGS += -g3 -fsanitize=address
else
	CFLAGS += -static
endif


CC = gcc

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
CFLAGS += -O2 -Wall -Wextra -fno-omit-frame-pointer $(IFLAGS) -lcrypt  # -lcriterion

.PHONY = all clean fclean re exec

TEST_DIR = tests

OBJ_DIR = objs
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

all: $(TARGET) $(TARGET_HASHER)

$(TARGET): $(OBJ_C)
	$(CC) -o $@ $(CFLAGS) $^
	sudo chown root:root $(TARGET)
	sudo chmod u+s $(TARGET)

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $<

clean:
	find $(OBJ_DIR) -type f -name '*.o' -exec rm -fv {} \;

fclean: clean
	rm -fv $(TARGET)

re: fclean all

test:
	bash test.sh

unit-tests: $(OBJ_C)
	$(CC) -o unit-tests $(CFLAGS) $^ #-lcriterion
	./unit-tests

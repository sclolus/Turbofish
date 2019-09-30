TARGET = program_test

SRC_C += main test_u_convert
VPATH += src
HEADERS += include/main.h

CC = gcc

CFLAGS += -m32 -g -O0 -fsanitize=address -Wall -Wextra

OBJ_DIR = obj
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

.PHONY = all clean fclean re exec libc/lib.a

all: build_libc $(TARGET)

$(TARGET): $(OBJ_C) libc/lib.a
	$(CC) $(CFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) $(CFLAGS) -Iinclude -Ilibc/include -c -o $@ $<

clean:
	make -C libc clean
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean:
	make -C libc fclean
	find $(OBJ_DIR) -type f -name '*.o' -delete
	rm -f $(TARGET)

build_libc:
	make -C libc

re: fclean all

exec: all
	./$(TARGET)

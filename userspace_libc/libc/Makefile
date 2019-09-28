
TARGET = lib.a
CC = gcc
AR = ar
RANLIB = ranlib

SRC_C += beacon op_main modifiers extract_args \
	s_string s_char s_numeric_u s_numeric_i \
	s_pointer s_logical_b s_logical_o s_logical_xmin s_logical_xmaj	\
	cast buffer norme \

VPATH += src/stdio/printf src/stdio/
HEADERS += libc.h
PRIVATE_HEADERS += internal_printf.h

SRC_C += ifc_entries ifc_parse_chain ifc_convert \
ifc_float ifc_char ifc_string ifc_pointer ifc_logical_x ifc_logical_o \
ifc_numeric_d ifc_numeric_i ifc_numeric_u

VPATH += src/stdio/scanf src/stdio/
PRIVATE_HEADERS += ifc.h

CFLAGS += -m32 -g -O0 -fsanitize=address -Wall -Wextra -fno-omit-frame-pointer -masm=intel $(IFLAGS)

OBJ_DIR = obj
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

HEADERS := $(addprefix include/, $(HEADERS))

.PHONY = all clean fclean re exec

all: $(TARGET)

$(TARGET): $(OBJ_C)
	$(AR) rc $@ $^
	$(RANLIB) $@

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS) $(PRIVATE_HEADERS)
	$(CC) $(CFLAGS) -Iinclude -c -o $@ $<

clean:
	find $(OBJ_DIR) -type f -name '*.o' -exec rm -fv {} \;

fclean: clean
	rm -fv $(TARGET)

re: fclean all

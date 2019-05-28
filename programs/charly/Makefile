TARGET = Charly

SRC_C += main minilibc
SRC_ASM += start user_syscall
VPATH += src
HEADERS += minilibc.h

AS = nasm
CC = gcc

ASFLAGS = -f elf
CFLAGS += -m32 -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions -nostdinc
LDFLAGS = -m elf_i386
ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

.PHONY = all clean fclean re exec

all: $(TARGET)

$(TARGET): $(OBJ_ASM) $(OBJ_C)
	$(LD) $(LDFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS)
	$(AS) $(ASFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $<

clean:
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(TARGET)

re: fclean all

exec: all
	./$(TARGET)

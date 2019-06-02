TARGET = Wait

SRC_C += main
SRC_ASM += start
VPATH += src

AS = nasm
CC = gcc

ASFLAGS = -f elf
CFLAGS += -m32 -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions -nostdinc
LDFLAGS = -T linker.ld -m elf_i386
ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif
IFLAGS += -Ilibc/include

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

.PHONY = all clean fclean re exec

all: build_libc $(TARGET)

$(TARGET): $(OBJ_ASM) $(OBJ_C) libc/libc.a linker.ld
	$(LD) $(LDFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS)
	$(AS) $(ASFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) $(IFLAGS) -o $@ $<

clean:
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(TARGET)

re: fclean all

exec: all
	./$(TARGET)

build_libc:
	make -C libc all DEBUG=$(DEBUG) OPTIM=$(OPTIM)

TARGET = ConnectionlessSimpleTest

SRC_C += main

# --- Preserve this above part. It is stricly system ---
ifeq ($(GNU),yes)
SRC_ASM +=
else
SRC_ASM += start
endif

VPATH += src

AS = nasm
CC = gcc

ifeq ($(GNU),yes)
CFLAGS += -m32
# Add GNU libc includes
LDFLAGS = -m elf_i386 -dynamic-linker /lib/ld-linux.so.2 /usr/lib32/crt1.o /usr/lib32/crti.o /usr/lib32/crtn.o -lc
else
CFLAGS += -m32 -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions -nostdinc
LDFLAGS = -T linker.ld -m elf_i386
endif
ASFLAGS = -f elf
CFLAGS += -m32
ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

ifeq ($(GNU),yes)
	IFLAGS +=
else
	IFLAGS += -Ilibc/include
endif

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

.PHONY = all clean fclean re exec build_libc

ifeq ($(GNU),yes)
all: $(TARGET)
else
all: build_libc $(TARGET)
endif

ifeq ($(GNU),yes)
$(TARGET): $(OBJ_ASM) $(OBJ_C)
	$(LD) $(LDFLAGS) -o $@ $^
else
$(TARGET): $(OBJ_ASM) $(OBJ_C) libc/libc.a
	$(LD) $(LDFLAGS) -o $@ $^
endif

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

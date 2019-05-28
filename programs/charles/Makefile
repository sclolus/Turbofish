TARGET = Charles

SRC_ASM += start
VPATH += src

AS = nasm

ASFLAGS = -f elf
LDFLAGS = -m elf_i386
ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))

.PHONY = all clean fclean re exec

all: $(TARGET)

$(TARGET): $(OBJ_ASM)
	$(LD) $(LDFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: %.asm Makefile
	$(AS) $(ASFLAGS) -o $@ $<

clean:
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(TARGET)

re: fclean all

exec: all
	./$(TARGET)

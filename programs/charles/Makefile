TARGET = Charles
ASM = nasm

ASMFLAGS = -f elf
LDFLAGS = -m elf_i386
ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

SRC_ASM += start
VPATH += src

OBJ_DIR = obj
OBJ_ASM_KERNEL = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))

.PHONY = all clean fclean re exec

all: $(TARGET)

$(TARGET): $(OBJ_ASM_KERNEL)
	$(LD) $(LDFLAGS) -o $@ $^

$(OBJ_DIR)/%.o: %.asm Makefile
	$(ASM) $(ASMFLAGS) -o $@ $<

clean:
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(TARGET)

re: fclean all

exec: all
	./$(TARGET)

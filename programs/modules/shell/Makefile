# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

BUILD_DIR = bin

TARGET = shell
OUTPUT = $(addprefix $(BUILD_DIR)/, $(TARGET))

VPATH += src

CC = i686-turbofish-gcc

SRC_C = termios

OBJ_DIR = obj
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

.PHONY = all clean fclean re exec

# Rust stuff here:
arch := ivybridge-cpu
target := $(arch)
ifeq ($(DEBUG),yes)
	rust_archive = ../target/$(target)/debug/lib$(TARGET).a
else
	rust_archive = ../target/$(target)/release/lib$(TARGET).a
endif

all: $(OUTPUT)

$(OUTPUT): $(OBJ_ASM) $(OBJ_C) $(rust_archive)
	$(CC) -Wl,--gc-sections -o $@ $^ -z muldefs
	cp -rv --parents $@ $(SYSTEM_ROOT)

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) $(IFLAGS) -o $@ $<

clean:
	cargo clean
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(OUTPUT)

re: fclean all

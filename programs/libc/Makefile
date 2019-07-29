TARGET = libc.a
SYSROOT = ~/cross/sysroot/

IFLAGS = -Iinclude
HEADERS = include/i386.h

SRC_C += errno
VPATH += src/errno
HEADERS += include/errno.h

SRC_C += memset bzero memcpy memccpy memmove memchr memcmp \
	strlen strcpy strncpy strcat strncat strlcat \
	strchr strrchr strstr strnstr strcmp strncmp \
	atoi toupper \
	strclr striter striteri \
	strequ strnequ \
	putendl putnbr putnbr_base \
	aligned_memcpy aligned_bzero strdup \
	strcasecmp strcspn strpbrk strspn strtod strtok strtoll strtoull strchrnul strerror
VPATH += src/string
HEADERS += include/string.h

SRC_C += beacon op_main modifiers extract_args \
	s_string s_char s_numeric_u s_numeric_i \
	s_pointer s_logical_b s_logical_o s_logical_xmin s_logical_xmaj	\
	cast buffer norme \
	stdio _doprnt putchar puts
VPATH += src/stdio/printf src/stdio/
HEADERS += include/stdio.h internal_printf.h

SRC_C += ctor mem_syscall \
	size_fn \
	index \
	debug_show_alloc debug_display_nodes \
	node_custom_allocator node_custom_deallocator \
	cmp_range cmp_fixe \
	free_record \
	allocator \
	reallocator \
	deallocator \
	main_prototypes \
	trace \
	sizeof_object \
	alloc_btree_ctor alloc_btree_get_node alloc_btree_apply_infix alloc_btree_memory_move alloc_btree_atomics_op \
	alloc_btree_insert_rnb_node alloc_btree_delete_rnb_node alloc_btree_insert_strategy alloc_btree_rotation_node \
	alloc_btree_try_to_insert alloc_btree_family_node alloc_btree_get_last_valid alloc_btree_delete_node \
	alloc_btree_insert_node alloc_btree_swap_content alloc_btree_limit_items alloc_btree_delete_strategy \
	alloc_btree_get_neighbours
VPATH += src/stdlib/alloc src/stdlib/alloc/alloc_btree
HEADERS += include/stdlib.h main_headers.h alloc_btree_internal_header.h

SRC_C += exit exit_qemu atexit env abort abs qsort _exit
VPATH += src/stdlib
HEADERS += include/stdlib.h

SRC_C += read write fork mmap munmap mprotect sleep getuid getpid close unlink pause reboot shutdown execve getpgid getpgrp setpgid getppid
VPATH += src/unistd
HEADERS += include/unistd.h

SRC_C += mmap munmap mprotect
VPATH += src/sys/mman 
HEADERS += include/sys/mman.h

SRC_C += mkfifo
VPATH += src/sys/stat 
HEADERS += include/sys/stat.h

SRC_ASM += clone
SRC_C += sched
VPATH += src/sched
HEADERS += include/sched.h

SRC_C += nanosleep
VPATH += src/time
HEADERS += include/time.h

SRC_C += signal kill raise killpg \
		sigemptyset sigfillset sigaddset sigdelset \
		strsignal \
        sigprocmask

VPATH += src/signal
HEADERS += include/signal.h

SRC_C += wait
VPATH += src/wait
HEADERS += include/wait.h

SRC_C += socket bind connect listen accept send recv sendto recvfrom shutdown
VPATH += src/socket
HEADERS += include/sys/socket.h include/sys/un.h

SRC_C += gettext locale
VPATH += src/locale
HEADERS += include/locale.h

SRC_C += isdigit isspace isblank iscntrl isgraph islower ispunct isupper isxdigit isalpha isalnum isascii isprint _tolower
VPATH += src/ctype
HEADERS += include/ctype.h

VPATH += src/setjmp
SRC_ASM += setjmp
HEADERS += include/setjmp.h

SRC_ASM += user_syscall
VPATH += src
HEADERS += include/user_syscall.h

AS = nasm
CC = i686-turbofish-gcc

ASFLAGS = -f elf
CFLAGS += -masm=intel $(IFLAGS)  -nostdlib -fno-builtin  -fno-stack-protector -nodefaultlibs -fno-exceptions #-nostdinc #$(IFLAGS)
#ifeq ($(DEBUG),yes)
#	CFLAGS += -m32 -std=gnu99 -Wextra -Wall -g -O0
#	ASFLAGS += -g
#else ifeq ($(OPTIM),yes)
#	# -fno-omit-frame-pointer keep good call trace for panic()
#	CFLAGS += -std=gnu99 -Wextra -Wall -O3 -fno-omit-frame-pointer
#else
#	CFLAGS += -v -std=gnu99 -Wextra -Wall
#endif

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

.PHONY = all clean fclean re exec

all: $(TARGET)

$(TARGET): $(OBJ_ASM) $(OBJ_C)
	ar rc $@ $^
	ranlib $@

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS)
	$(AS) $(ASFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) -o $@ $<

crt0.o: src/start.asm
	$(AS) $(ASFLAGS) -o crt0.o src/start.asm

clean:
	find $(OBJ_DIR) -type f -name '*.o' -delete

fclean: clean
	rm -f $(TARGET)

re: fclean all

exec: all
	./$(TARGET)

install:
	cp libc.a $(SYSROOT)/usr/lib/.
	cp -r include/* $(SYSROOT)/usr/include/
	cp crt0.o $(SYSROOT)/usr/lib/crt0.o

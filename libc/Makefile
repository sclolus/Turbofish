# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

TARGET_UNIT_TESTS= unit-tests
TARGET_LIBC= libc.a
ifeq ($(unit-tests),yes)
	LD_LIBRARY_PATH := /usr/local/lib:$(LD_LIBRARY_PATH)
	TARGET = $(TARGET_UNIT_TESTS)
	AS = as
	CC = gcc
	AR = ar
	RANLIB = ranlib
	CFLAGS += -D UNIT_TESTS -g3 -fsanitize=address
	ASFLAGS = --32
	IFLAGS =
else
	TARGET = $(TARGET_LIBC)
	AS = i686-turbofish-as
	CC = i686-turbofish-gcc
	AR = i686-turbofish-ar
	RANLIB = i686-turbofish-ranlib
	IFLAGS = -Iinclude
	ASFLAGS = --32
endif

# non-turbofish specific program. Use carefully
NASM = nasm

VPATH += src
SRC_C += libc_ctor
SRC_ASM += user_syscall
PRIVATE_HEADERS += include/user_syscall.h

TESTED_SRC_C = getenv setenv unsetenv clearenv execl


SRC_C += memset bzero memcpy memccpy memmove memchr memcmp \
	strlen strcpy strncpy strcat strncat strlcat \
	strchr strrchr strstr strnstr strcmp strncmp \
	atoi toupper \
	strclr striter striteri \
	strequ strnequ \
	putendl putnbr putnbr_base \
	aligned_memcpy aligned_bzero strdup \
	strcasecmp strcspn strpbrk strspn strtod strtok strtoll strtoull strchrnul strerror \
	strcoll
VPATH += src/string
HEADERS += string.h

SRC_C += beacon op_main modifiers extract_args \
	s_string s_char s_numeric_u s_numeric_i \
	s_pointer s_logical_b s_logical_o s_logical_xmin s_logical_xmaj	\
	cast buffer norme perror \
	stdio _doprnt putchar puts getc fopen fclose \
	getline getdelim getchar getchar_unlocked
VPATH += src/stdio/printf src/stdio/
HEADERS += stdio.h
PRIVATE_HEADERS += internal_printf.h

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
HEADERS += stdlib.h
PRIVATE_HEADERS += main_headers.h alloc_btree_internal_header.h

SRC_C += exit exit_qemu atexit getenv setenv unsetenv clearenv abort abs qsort _Exit

VPATH += src/stdlib
HEADERS += stdlib.h

SRC_C += read write fork mmap munmap mprotect sleep getuid getpid close unlink pause reboot shutdown execve getpgid getpgrp setpgid getppid tcsetpgrp tcgetpgrp tcgetpgrp tcsetpgrp isatty getegid geteuid getgid chdir getcwd setegid seteuid setgid setgroups setuid seteuid setegid setgroups getgroups getopt getpass _exit lseek execl

SRC_C += dup dup2 pipe

VPATH += src/unistd
HEADERS += unistd.h

SRC_C += mmap munmap mprotect
VPATH += src/sys/mman
HEADERS += sys/mman.h

SRC_C += stat lstat fstat mkfifo umask
VPATH += src/sys/stat
HEADERS += sys/stat.h

SRC_C += gettimeofday settimeofday
VPATH += src/sys/time
HEADERS += sys/time.h

SRC_ASM += clone
SRC_C += sched
VPATH += src/sched
HEADERS += sched.h

SRC_C += nanosleep localtime time gmtime
VPATH += src/time
HEADERS += time.h

SRC_C += signal sigaction kill raise killpg \
		sigemptyset sigfillset sigaddset sigdelset \
		strsignal \
        sigprocmask sigsuspend sigismember

VPATH += src/signal
HEADERS += signal.h

SRC_C += wait wait3 wait4
VPATH += src/wait
HEADERS += wait.h sys/wait.h

SRC_C += socket bind connect listen accept send recv sendto recvfrom shutdown
VPATH += src/socket
HEADERS += sys/socket.h sys/un.h

SRC_C += gettext locale
VPATH += src/locale
HEADERS += locale.h

SRC_C += tcgetattr tcsetattr
VPATH += src/termios
HEADERS += termios.h

SRC_C += opendir readdir closedir
VPATH += src/dirent
HEADERS += dirent.h

SRC_C += fcntl open
VPATH += src/fcntl
HEADERS += fcntl.h

SRC_C += isdigit isspace isblank iscntrl isgraph islower ispunct isupper isxdigit isalpha isalnum isascii isprint _tolower tolower
VPATH += src/ctype
HEADERS += ctype.h

VPATH += src/math
SRC_ASM += pow.c
HEADERS += math.h

VPATH += src/setjmp
SRC_ASM += setjmp
HEADERS += setjmp.h

VPATH += src/math
SRC_NASM += round
HEADERS += math.h

SRC_C += strsplit array parse_2d_file
VPATH += src/tools
HEADERS += tools.h

SRC_C += getpwnam getpwuid
VPATH += src/pwd
HEADERS += pwd.h

SRC_C += getgrnam getgrgid
VPATH += src/grp
HEADERS += grp.h

SRC_C += strcasecmp
VPATH += src/strings
HEADERS += strings.h

HEADERS += sys/types.h errno.h stdint.h sys/param.h inttypes.h limits.h assert.h custom.h
# bullshit headers needed by coreutils and dash
HEADERS += sys/select.h getopt.h sys/resource.h sys/ioctl.h sys/times.h libintl.h

CFLAGS += -O2 -Wall -Wextra -fno-omit-frame-pointer -masm=intel $(IFLAGS)

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))
TESTED_OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(TESTED_SRC_C)))))

# non-turbofish specific
OBJ_NASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_NASM)))))

SRC_C_RUNTIME = crt0.s crti.s crtn.s crtbegin.s crtend.s
OBJ_C_RUNTIME = $(addprefix $(OBJ_DIR)/crt/, $(addsuffix .o, $(basename $(notdir $(SRC_C_RUNTIME)))))

HEADERS := $(addprefix include/, $(HEADERS))
SYSROOT_HEADERS = $(addprefix $(TOOLCHAIN_SYSROOT)/usr/, $(HEADERS))

.PHONY = all clean fclean re exec

all: $(TARGET) $(OBJ_C_RUNTIME) $(SYSROOT_HEADERS)

$(TOOLCHAIN_SYSROOT)/usr/include/%: include/%
	@mkdir -pv $(dir $@)
	@cp -v $< $@

$(TARGET_LIBC): $(OBJ_C_RUNTIME) $(OBJ_NASM) $(OBJ_ASM) $(OBJ_C)
	$(AR) rc $@ $^
	$(RANLIB) $@
	cp -v libc.a $(TOOLCHAIN_SYSROOT)/usr/lib/.
	cp -v $(OBJ_C_RUNTIME) $(TOOLCHAIN_SYSROOT)/usr/lib/.

$(TARGET_UNIT_TESTS): $(TESTED_OBJ_C)
	$(CC) $(CFLAGS) -o $@ $^ -lcriterion

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS) $(PRIVATE_HEADERS) include include/sys
	$(CC) -c $(CFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.s Makefile $(HEADERS) $(PRIVATE_HEADERS) include include/sys
	$(AS) $(ASFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS) $(PRIVATE_HEADERS) include include/sys
	$(NASM) -f elf -o $@ $<

# Compile all the C runtime object files
obj/crt/%.o: src/crt/%.s
	$(AS) $(ASFLAGS) -o $@ $<

clean:
	find $(OBJ_DIR) -type f -name '*.o' -exec rm -fv {} \;

fclean: clean
	rm -fv $(TARGET_LIBC)
	rm -fv $(TARGET_UNIT_TESTS)

re: fclean all

exec: all
	./$(TARGET_LIBC)

install:
	cp -v libc.a $(TOOLCHAIN_SYSROOT)/usr/lib/.
	cp -v $(OBJ_C_RUNTIME) $(TOOLCHAIN_SYSROOT)/usr/lib/.
	cp -rv include/* $(TOOLCHAIN_SYSROOT)/usr/include/

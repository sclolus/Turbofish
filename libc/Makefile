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
	IFLAGS = -Iinclude/ltrace -Iinclude/tools
else
	TARGET = $(TARGET_LIBC)
	AS = i686-turbofish-as
	CC = i686-turbofish-gcc
	AR = i686-turbofish-ar
	RANLIB = i686-turbofish-ranlib
	IFLAGS = -Iinclude -I./include/ltrace -Iinclude/tools
	ASFLAGS = --32
endif

# non-turbofish specific program. Use carefully
NASM = nasm

VPATH += src
SRC_C += libc_ctor
SRC_ASM += user_syscall
PRIVATE_HEADERS += include/user_syscall.h

TESTED_SRC_C = ifc_entries strsplit array getenv setenv unsetenv clearenv execl getmntent


SRC_C += memset bzero memcpy memccpy memmove memchr memcmp \
	strlen strcpy strncpy strcat strncat strlcat \
	strchr strrchr strstr strnstr strcmp strncmp \
	atoi toupper \
	strclr striter striteri \
	strequ strnequ \
	putendl putnbr putnbr_base \
	aligned_memcpy aligned_bzero strdup \
	strcspn strpbrk strspn strtod strtok strtoll strtoul strtoull strchrnul strerror \
	strcoll
VPATH += src/string
HEADERS += string.h

SRC_C += beacon op_main modifiers extract_args \
	s_string s_char s_numeric_u s_numeric_i \
	s_pointer s_logical_b s_logical_o s_logical_xmin s_logical_xmaj	s_float \
	cast buffer norme perror \
	stdio _doprnt putchar puts getc fopen fclose \
	getline getdelim getchar getchar_unlocked fgets fgetc rename getc_unlocked \
	sscanf fread rewind \

VPATH += src/stdio/printf src/stdio/
HEADERS += stdio.h
PRIVATE_HEADERS += internal_printf.h

VPATH += src/stdio/scanf/
SRC_C += ifc_char \
	ifc_convert \
	ifc_entries \
	ifc_float \
	ifc_logical_o \
	ifc_logical_x \
	ifc_numeric_d \
	ifc_numeric_i \
	ifc_numeric_u \
	ifc_parse_chain \
	ifc_pointer \
	ifc_string \


SRC_C += fpending
VPATH += src/stdio_ext/
HEADERS += stdio_ext.h

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

SRC_C += bsearch exit exit_qemu atexit getenv setenv unsetenv clearenv abort abs qsort _Exit rand
SRC_ASM += _rdrand

VPATH += src/stdlib
HEADERS += stdlib.h

SRC_C += read write fork mmap munmap mprotect sleep getuid getpid close unlink pause reboot shutdown execve getpgid getpgrp setpgid getppid tcsetpgrp tcgetpgrp tcgetpgrp tcsetpgrp isatty getegid geteuid getgid chdir getcwd setegid seteuid setgid setgroups setuid seteuid setegid setgroups getgroups getopt getpass _exit lseek execl execv access chown fchown link rmdir getpagesize symlink dup dup2 pipe execvp sync readlink sysconf gethostname sethostname

VPATH += src/unistd
HEADERS += unistd.h

SRC_C += getopt_long
VPATH += src/getopt_ext
HEADERS += getopt.h getopt_ext.h

SRC_C += mmap munmap mprotect
VPATH += src/sys/mman
HEADERS += sys/mman.h

SRC_C += stat lstat fstat mkfifo umask chmod fchmod mkdir mknod
VPATH += src/sys/stat
HEADERS += sys/stat.h

SRC_C += select fd_clr fd_isset fd_set fd_zero
VPATH += src/sys/select/
VPATH += sys/select.h


SRC_C += statfs fstatfs
VPATH += src/sys/statfs
HEADERS += sys/statfs.h

SRC_C += statvfs fstatvfs
VPATH += src/sys/statvfs
HEADERS += sys/statvfs.h

SRC_C += gettimeofday settimeofday times
VPATH += src/sys/time
HEADERS += sys/time.h

SRC_C += mount umount
VPATH += src/sys/mount
HEADERS += sys/mount.h

SRC_ASM += clone
SRC_C += sched sched_setscheduler
VPATH += src/sched
HEADERS += sched.h

SRC_C += nanosleep localtime localtime_r time gmtime ctime asctime mktime strftime tzset
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
SRC_NASM += __pow nanf # Woot ?
SRC_NASM += round roundf sinf cosf sqrt sqrtf tan tanf atan atanf atan2 atan2f
SRC_C += floor floorf
HEADERS += math.h

SRC_C += strsplit array parse_2d_file
VPATH += src/tools
HEADERS += tools/tools.h

SRC_C += getpwnam getpwuid
VPATH += src/pwd
HEADERS += pwd.h

SRC_C += utime
VPATH += src/utime
HEADERS += utime.h

SRC_C += getgrnam getgrgid
VPATH += src/grp
HEADERS += grp.h

SRC_C += strcasecmp strncasecmp
VPATH += src/strings
HEADERS += strings.h

SRC_C += setmntent getmntent endmntent addmntent hasmntopt
VPATH += src/mntent
HEADERS += mntent.h


SRC_C += is_ptr_valid
VPATH += src/custom

SRC_C += rmmod insmod lsmod
VPATH += src/mod
HEADERS += mod.h

SRC_C += setpriority getpriority
VPATH += src/sys/resource
HEADERS += sys/resource.h

SRC_C += ioctl
VPATH += src/stropts
HEADERS += stropts.h  sys/ioctl.h


SRC_C += is_ptr_valid
VPATH += src/custom

HEADERS += bits/wordsize.h
HEADERS += utmp.h # For procps package.
HEADERS += paths.h
HEADERS += sys/sysmacros.h


HEADERS += sys/types.h errno.h stdint.h sys/param.h inttypes.h limits.h assert.h custom.h
# bullshit headers needed by coreutils, dash, and procps
HEADERS += getopt_ext.h malloc.h getopt.h  sys/times.h libintl.h mntent.h netinet/in.h link.h elf.h sys/ioctl.h

HEADERS += ltrace/ltrace.h

CFLAGS += -O2 -Wall -Wextra -fno-omit-frame-pointer -masm=intel $(IFLAGS)
ifeq ($(trace),yes)
	CFLAGS += -D LTRACE
endif

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))
TESTED_OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(TESTED_SRC_C)))))

# non-turbofish specific
OBJ_NASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_NASM)))))

SRC_C_RUNTIME = crt0.s crti.s crtn.s
OBJ_C_RUNTIME = $(addprefix $(OBJ_DIR)/crt/, $(addsuffix .o, $(basename $(notdir $(SRC_C_RUNTIME)))))

HEADERS := $(addprefix include/, $(HEADERS))
SYSROOT_HEADERS = $(addprefix $(TOOLCHAIN_SYSROOT)/usr/, $(HEADERS))

.PHONY = all clean fclean re exec

# CRTBEGIN_OBJ:=$(shell $(CC) $(CFLAGS) -print-file-name=crtbegin.o)
# CRTEND_OBJ:=$(shell $(CC) $(CFLAGS) -print-file-name=crtend.o)

all: $(OBJ_C_RUNTIME) $(TARGET) $(SYSROOT_HEADERS)

$(TOOLCHAIN_SYSROOT)/usr/include/%: include/%
	@mkdir -pv $(dir $@)
	@cp -v $< $@

$(TARGET_LIBC): $(OBJ_NASM) $(OBJ_ASM) $(OBJ_C)
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

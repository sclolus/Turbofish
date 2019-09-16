# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

BUILD_DIR = bin
VPATH += src

AS = i686-turbofish-ld
CC = i686-turbofish-gcc

SRC_C += MasterDeepThought \
		DeepThought \
		ProcessGroup \
		signal/SignalSimple \
		signal/SignalSimpleDuo \
		execve/argv \
		execve/check_argv \
		execve/cannot_exec_directory \
		sigprocmask/sigprocmask \
		wait/wait \
		wait/wuntraced \
		munmap/munmap \
		mprotect/mprotect \
		mmap/mmap \
		isatty/isatty \
		atexit/atexit \
		pipe/pipe_fucker \
		pipe/pipe_fister \
		pipe/pipe_lorem_ipsum \
		math/roundf \
		math/pow \
		ctype/longlong \
		lseek/sda \
		lseek/lseek_return \
		dirent/dummy_root \
		read/read_pulp_fiction \
		execl/execl \
		is_str_bullshit/is_str_bullshit \
		write/create_write_read \
		access/access \
		unlink/unlink \
		unlink/unlink_multiple \
		unlink/unlink_orphan \
		umask/umask \
		dir/mkdir_exist \
		dir/mkdir \
		dir/rmdir_not_empty \
		symlink/symlink \
		link/link \
		rename/rename \
		rename/rename_dir \
		rename/rename_dir_exist \
		rename/rename_dir_not_empty \
		rename/rename_file_exist \
		statfs/statfs \
		statvfs/statvfs \
		fstatfs/fstatfs \
		fstatvfs/fstatvfs \
		chmod_tests/einval_mode \
		chmod_tests/chmod_normal \
		fchmod/einval_mode \
		fchmod/fchmod_normal \
		utime/utime_basic \
		chown_tests/chown_basic \
		fchown/fchown_basic \
		execve/bad_elf \
		fifo/fifo \
		open/open_fails_with_eaccess_basic

VPATH += src/open src/signal src/execve src/sigprocmask src/wait src/munmap src/mprotect src/mmap src/isatty src/atexit src/pipe src/math src/execl src/umask src/statvfs src/statfs src/fstatfs src/fstatvfs src/rename src/unlink src/dir src/symlink src/chmod_tests src/fchmod src/utime src/fchown src/chown_tests src/fchown

OBJ_DIR = obj
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(SRC_C)))

IFLAGS = -I include

CFLAGS = -Wall -Wextra -fno-omit-frame-pointer
ifeq ($(GNU),yes)
CFLAGS += -m32 -D GNU
CC = gcc
LIB = -lm
else
endif

TARGET_C = $(addprefix $(BUILD_DIR)/, $(SRC_C))

.PHONY = all clean fclean re

all: $(TARGET_C)

$(BUILD_DIR)/%: $(OBJ_DIR)/%.o Makefile $(LIBC_AR) obj/tools.o
	@mkdir -pv ./$(dir $@)
	$(CC) $(CFLAGS) -o $@ $< obj/tools.o $(LIB)
ifneq ($(GNU),yes)
	cp -rv --parents $@ $(SYSTEM_ROOT)
endif

$(OBJ_DIR)/%.o: %.c Makefile include/tools.h
	@mkdir -pv ./$(dir $@)
	$(CC) $(CFLAGS) $(IFLAGS) -c -o $@ $<

$(OBJ_C):

obj/tools.o:

clean:
	rm -fv $(TARGET_C)
	find $(OBJ_DIR)/ -name "*.o" -delete

fclean: clean

re: fclean all

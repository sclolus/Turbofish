# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

BUILD_DIR = bin
VPATH += src

# AS = i686-turbofish-ld
CC = i686-turbofish-gcc

SRC_C += Clone \
		ConnectionlessSimpleTest \
		ConnectionOrientedSimpleTest \
		Continue \
		ContinueHardcore \
		Csignal \
		DummyRead \
		GetArgs \
		Getuid \
		Kill \
		NoSaRestart \
		NoSaRestartMultiple \
		RecursiveSignal \
		SaRestart \
		SaRestartMultiple \
		SegFault \
		SignalSimpleDuoRecurse \
		SignalSimpleStopContinue \
		SignalStopContinueOverload \
		SonSendSignal \
		Timer \
		Ud2 \
		Wait \
		env \
		SetJmp \
		SigSuspend \
		SigPause \
		TcGetAttr \
		CanonicalRead \
		Loop \
		Segfaulter \
		PipeCreater \
		pipe_com1 \
		westoped \
		getenv \
		fish \
		more_fish \
		broken_fish \
		gettimeofday \
		time \
		fork_bomb \
		client_socket \
		server_socket \

CFLAGS = -Wall -Wextra -fno-omit-frame-pointer -I include
ifeq ($(GNU),yes)
CFLAGS += -m32 -D GNU
CC = gcc
else
endif

#ASFLAGS = -f elf

ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

TARGET_C = $(addprefix $(BUILD_DIR)/, $(SRC_C))

.PHONY = all clean fclean re install

all: $(TARGET_C)

$(BUILD_DIR)/%: %.c $(LIBC_AR) Makefile
	$(CC) $(CFLAGS) -o $@ $<
ifneq ($(GNU),yes)
	cp -rv --parents $@ $(SYSTEM_ROOT)
endif

clean:
	rm -fv $(TARGET_C)

fclean: clean

re: fclean all

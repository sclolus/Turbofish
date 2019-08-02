PATH  := $(PATH):/toolchain_turbofish/cross/bin/
SHELL := env PATH=$(PATH) /bin/bash

BUILD_DIR = build
VPATH += src
VPATH += src/posix_test_suite

# AS = i686-turbofish-ld
CC = i686-turbofish-gcc

SRC_C += Clone \
		ConnectionlessSimpleTest \
		ConnectionOrientedSimpleTest \
		Continue \
		ContinueHardcore \
		Csignal \
		DeepThought \
		DummyRead \
		GetArgs \
		Getuid \
		Kill \
		NoSaRestart \
		NoSaRestartMultiple \
		ProcessGroup \
		RecursiveSignal \
		SaRestart \
		SaRestartMultiple \
		SegFault \
		SignalSimple \
		SignalSimpleDuo \
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
		Loop


SRC_C += kill/1-1 \
		# kill/1-2 \
		# kill/2-1 \
		# kill/2-2 \
		# kill/3-1

SRC_C += signal/1-1 \
		signal/2-1 \
		signal/3-1 \
		signal/5-1 \
		signal/6-1 \
		signal/7-1

SRC_C += # sigprocmask/10-1 \
		# sigprocmask/12-1 \
		# sigprocmask/15-1 \
		# sigprocmask/17-core-buildonly \
		# sigprocmask/4-1 \
		# sigprocmask/5-1 \
		# sigprocmask/6-1 \
		# sigprocmask/7-1 \
		# sigprocmask/8-1 \
		# sigprocmask/8-2 \
		# sigprocmask/8-3 \
		# sigprocmask/9-1

CFLAGS = -Wall -Wextra -I include
ifeq ($(GNU),yes)
CFLAGS += -m32 -D GNU
CC = gcc
else
CFLAGS +=
endif

ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

TARGET_C = $(addprefix $(BUILD_DIR)/, $(SRC_C))
INSTALL_DIR = ../../rust_kernel/src/userland/

.PHONY = all clean fclean re

all: $(TARGET_C)

build/%: %.c Makefile
	$(CC) $(CFLAGS) -o $@ $<

install:
	cp -rv build/* $(INSTALL_DIR)

clean:
	rm -fv $(TARGET_C)

fclean: clean

re: fclean all

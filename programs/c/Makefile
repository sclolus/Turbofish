PATH  := $(PATH):/toolchain_turbofish/cross/bin/
SHELL := env PATH=$(PATH) /bin/bash

BUILD_DIR = build
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

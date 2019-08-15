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

SRC_C += sigaction/1-10 \
	sigaction/1-11 \
	sigaction/1-12 \
	sigaction/1-13 \
	sigaction/1-14 \
	sigaction/1-15 \
	sigaction/1-16 \
	sigaction/1-17 \
	sigaction/1-18 \
	sigaction/1-19 \
	sigaction/1-1 \
	sigaction/1-20 \
	sigaction/1-21 \
	sigaction/1-22 \
	sigaction/1-23 \
	sigaction/1-24 \
	sigaction/1-25 \
	sigaction/1-26 \
	sigaction/1-2 \
	sigaction/1-3 \
	sigaction/1-4 \
	sigaction/1-5 \
	sigaction/1-6 \
	sigaction/1-7 \
	sigaction/1-8 \
	sigaction/1-9 \
	sigaction/2-10 \
	sigaction/2-11 \
	sigaction/21-1 \
	sigaction/2-12 \
	sigaction/2-13 \
	sigaction/2-14 \
	sigaction/2-15 \
	sigaction/2-16 \
	sigaction/2-17 \
	sigaction/2-18 \
	sigaction/2-19 \
	sigaction/2-1 \
	sigaction/2-20 \
	sigaction/22-10 \
	sigaction/22-11 \
	sigaction/22-12 \
	sigaction/22-13 \
	sigaction/22-14 \
	sigaction/22-15 \
	sigaction/22-16 \
	sigaction/22-17 \
	sigaction/22-18 \
	sigaction/22-19 \
	sigaction/2-21 \
	sigaction/22-1 \
	sigaction/22-20 \
	sigaction/22-21 \
	sigaction/22-22 \
	sigaction/22-23 \
	sigaction/22-24 \
	sigaction/22-25 \
	sigaction/22-26 \
	sigaction/2-22 \
	sigaction/22-2 \
	sigaction/2-23 \
	sigaction/22-3 \
	sigaction/2-24 \
	sigaction/22-4 \
	sigaction/2-25 \
	sigaction/22-5 \
	sigaction/2-26 \
	sigaction/22-6 \
	sigaction/22-7 \
	sigaction/22-8 \
	sigaction/22-9 \
	sigaction/2-2 \
	sigaction/2-3 \
	sigaction/2-4 \
	sigaction/25-10 \
	sigaction/25-11 \
	sigaction/25-12 \
	sigaction/25-13 \
	sigaction/25-14 \
	sigaction/25-15 \
	sigaction/25-16 \
	sigaction/25-17 \
	sigaction/25-18 \
	sigaction/25-19 \
	sigaction/25-1 \
	sigaction/25-20 \
	sigaction/25-21 \
	sigaction/25-22 \
	sigaction/25-23 \
	sigaction/25-24 \
	sigaction/25-25 \
	sigaction/25-26 \
	sigaction/25-2 \
	sigaction/25-3 \
	sigaction/25-4 \
	sigaction/25-5 \
	sigaction/25-6 \
	sigaction/25-7 \
	sigaction/25-8 \
	sigaction/25-9 \
	sigaction/2-5 \
	sigaction/2-6 \
	sigaction/2-7 \
	sigaction/2-8 \
	sigaction/2-9 \
	sigaction/3-10 \
	sigaction/3-11 \
	sigaction/3-12 \
	sigaction/3-13 \
	sigaction/3-14 \
	sigaction/3-15 \
	sigaction/3-16 \
	sigaction/3-17 \
	sigaction/3-18 \
	sigaction/3-19 \
	sigaction/3-1 \
	sigaction/3-20 \
	sigaction/3-21 \
	sigaction/3-22 \
	sigaction/3-23 \
	sigaction/3-24 \
	sigaction/3-25 \
	sigaction/3-26 \
	sigaction/3-2 \
	sigaction/3-3 \
	sigaction/3-4 \
	sigaction/3-5 \
	sigaction/3-6 \
	sigaction/3-7 \
	sigaction/3-8 \
	sigaction/3-9 \
	sigaction/4-100 \
	sigaction/4-101 \
	sigaction/4-102 \
	sigaction/4-103 \
	sigaction/4-104 \
	sigaction/4-10 \
	sigaction/4-11 \
	sigaction/4-12 \
	sigaction/4-13 \
	sigaction/4-14 \
	sigaction/4-15 \
	sigaction/4-16 \
	sigaction/4-17 \
	sigaction/4-18 \
	sigaction/4-19 \
	sigaction/4-1 \
	sigaction/4-20 \
	sigaction/4-21 \
	sigaction/4-22 \
	sigaction/4-23 \
	sigaction/4-24 \
	sigaction/4-25 \
	sigaction/4-26 \
	sigaction/4-27 \
	sigaction/4-28 \
	sigaction/4-29 \
	sigaction/4-2 \
	sigaction/4-30 \
	sigaction/4-31 \
	sigaction/4-32 \
	sigaction/4-33 \
	sigaction/4-34 \
	sigaction/4-35 \
	sigaction/4-36 \
	sigaction/4-37 \
	sigaction/4-38 \
	sigaction/4-39 \
	sigaction/4-3 \
	sigaction/4-40 \
	sigaction/4-41 \
	sigaction/4-42 \
	sigaction/4-43 \
	sigaction/4-44 \
	sigaction/4-45 \
	sigaction/4-46 \
	sigaction/4-47 \
	sigaction/4-48 \
	sigaction/4-49 \
	sigaction/4-4 \
	sigaction/4-50 \
	sigaction/4-51 \
	sigaction/4-52 \
	sigaction/4-53 \
	sigaction/4-54 \
	sigaction/4-55 \
	sigaction/4-56 \
	sigaction/4-57 \
	sigaction/4-58 \
	sigaction/4-59 \
	sigaction/4-5 \
	sigaction/4-60 \
	sigaction/4-61 \
	sigaction/4-62 \
	sigaction/4-63 \
	sigaction/4-64 \
	sigaction/4-65 \
	sigaction/4-66 \
	sigaction/4-67 \
	sigaction/4-68 \
	sigaction/4-69 \
	sigaction/4-6 \
	sigaction/4-70 \
	sigaction/4-71 \
	sigaction/4-72 \
	sigaction/4-73 \
	sigaction/4-74 \
	sigaction/4-75 \
	sigaction/4-76 \
	sigaction/4-77 \
	sigaction/4-78 \
	sigaction/4-79 \
	sigaction/4-7 \
	sigaction/4-80 \
	sigaction/4-81 \
	sigaction/4-82 \
	sigaction/4-83 \
	sigaction/4-84 \
	sigaction/4-85 \
	sigaction/4-86 \
	sigaction/4-87 \
	sigaction/4-88 \
	sigaction/4-89 \
	sigaction/4-8 \
	sigaction/4-90 \
	sigaction/4-91 \
	sigaction/4-92 \
	sigaction/4-93 \
	sigaction/4-94 \
	sigaction/4-95 \
	sigaction/4-96 \
	sigaction/4-97 \
	sigaction/4-98 \
	sigaction/4-99 \
	sigaction/4-9 \
	sigaction/6-10 \
	sigaction/6-11 \
	sigaction/6-12 \
	sigaction/6-13 \
	sigaction/6-14 \
	sigaction/6-15 \
	sigaction/6-16 \
	sigaction/6-17 \
	sigaction/6-18 \
	sigaction/6-19 \
	sigaction/6-1 \
	sigaction/6-20 \
	sigaction/6-21 \
	sigaction/6-22 \
	sigaction/6-23 \
	sigaction/6-24 \
	sigaction/6-25 \
	sigaction/6-26 \
	sigaction/6-2 \
	sigaction/6-3 \
	sigaction/6-4 \
	sigaction/6-5 \
	sigaction/6-6 \
	sigaction/6-7 \
	sigaction/6-8 \
	sigaction/6-9 \
	sigaction/8-10 \
	sigaction/8-11 \
	sigaction/8-12 \
	sigaction/8-13 \
	sigaction/8-14 \
	sigaction/8-15 \
	sigaction/8-16 \
	sigaction/8-17 \
	sigaction/8-18 \
	sigaction/8-19 \
	sigaction/8-1 \
	sigaction/8-20 \
	sigaction/8-21 \
	sigaction/8-22 \
	sigaction/8-23 \
	sigaction/8-24 \
	sigaction/8-25 \
	sigaction/8-26 \
	sigaction/8-2 \
	sigaction/8-3 \
	sigaction/8-4 \
	sigaction/8-5 \
	sigaction/8-6 \
	sigaction/8-7 \
	sigaction/8-8 \
	sigaction/8-9 
	# sigaction/10-1 \ sigaction/11-1 \ sigaction/12-10 \ sigaction/12-11 \ sigaction/12-12 \ sigaction/12-13 \ sigaction/12-14 \ sigaction/12-15 \ sigaction/12-16 \ sigaction/12-17 \ sigaction/12-18 \ sigaction/12-19 \ sigaction/12-1 \ sigaction/12-20 \ sigaction/12-21 \ sigaction/12-22 \ sigaction/12-23 \ sigaction/12-24 \ sigaction/12-25 \ sigaction/12-26 \ sigaction/12-27 \ sigaction/12-28 \ sigaction/12-29 \ sigaction/12-2 \ sigaction/12-30 \ sigaction/12-31 \ sigaction/12-32 \ sigaction/12-33 \ sigaction/12-34 \ sigaction/12-35 \ sigaction/12-36 \ sigaction/12-37 \ sigaction/12-38 \ sigaction/12-39 \ sigaction/12-3 \ sigaction/12-40 \ sigaction/12-41 \ sigaction/12-42 \ sigaction/12-43 \ sigaction/12-44 \ sigaction/12-45 \ sigaction/12-46 \ sigaction/12-47 \ sigaction/12-48 \ sigaction/12-49 \ sigaction/12-4 \ sigaction/12-50 \ sigaction/12-51 \ sigaction/12-52 \ sigaction/12-6 \ sigaction/12-7 \ sigaction/12-8 \ sigaction/12-9 \ sigaction/12-5 \ sigaction/13-10 \ sigaction/13-11 \ sigaction/13-12 \ sigaction/13-13 \ sigaction/13-14 \ sigaction/13-15 \ sigaction/13-16 \ sigaction/13-17 \ sigaction/13-18 \ sigaction/13-19 \ sigaction/13-1 \ sigaction/13-20 \ sigaction/13-21 \ sigaction/13-22 \ sigaction/13-23 \ sigaction/13-24 \ sigaction/13-25 \ sigaction/13-26 \ sigaction/13-2 \ sigaction/13-3 \ sigaction/13-4 \ sigaction/13-5 \ sigaction/13-6 \ sigaction/13-7 \ sigaction/13-8 \ sigaction/13-9 \ sigaction/16-10 \ sigaction/16-11 \ sigaction/16-12 \ sigaction/16-13 \ sigaction/16-14 \ sigaction/16-15 \ sigaction/16-16 \ sigaction/16-17 \ sigaction/16-18 \ sigaction/16-19 \ sigaction/16-1 \ sigaction/16-20 \ sigaction/16-21 \ sigaction/16-22 \ sigaction/16-23 \ sigaction/16-24 \ sigaction/16-25 \ sigaction/16-26 \ sigaction/16-2 \ sigaction/16-3 \ sigaction/16-4 \ sigaction/16-5 \ sigaction/16-6 \ sigaction/16-7 \ sigaction/16-8 \ sigaction/16-9 \ sigaction/17-10 \ sigaction/17-11 \ sigaction/17-12 \ sigaction/17-13 \ sigaction/17-14 \ sigaction/17-15 \ sigaction/17-16 \ sigaction/17-17 \ sigaction/17-18 \ sigaction/17-19 \ sigaction/17-1 \ sigaction/17-20 \ sigaction/17-21 \ sigaction/17-22 \ sigaction/17-23 \ sigaction/17-24 \ sigaction/17-25 \ sigaction/17-26 \ sigaction/17-2 \ sigaction/17-3 \ sigaction/17-4 \ sigaction/17-5 \ sigaction/17-6 \ sigaction/17-7 \ sigaction/17-8 \ sigaction/17-9 \ sigaction/18-10 \ sigaction/18-11 \ sigaction/18-12 \ sigaction/18-13 \ sigaction/18-14 \ sigaction/18-15 \ sigaction/18-16 \ sigaction/18-17 \ sigaction/18-18 \ sigaction/18-19 \ sigaction/18-1 \ sigaction/18-20 \ sigaction/18-21 \ sigaction/18-22 \ sigaction/18-23 \ sigaction/18-24 \ sigaction/18-25 \ sigaction/18-26 \ sigaction/18-2 \ sigaction/18-3 \ sigaction/18-4 \ sigaction/18-5 \ sigaction/18-6 \ sigaction/18-7 \ sigaction/18-8 \ sigaction/18-9 \ sigaction/19-10 \ sigaction/19-11 \ sigaction/19-12 \ sigaction/19-13 \ sigaction/19-14 \ sigaction/19-15 \ sigaction/19-16 \ sigaction/19-17 \ sigaction/19-18 \ sigaction/19-19 \ sigaction/19-1 \ sigaction/19-20 \ sigaction/19-21 \ sigaction/19-22 \ sigaction/19-23 \ sigaction/19-24 \ sigaction/19-25 \ sigaction/19-26 \ sigaction/19-2 \ sigaction/19-3 \ sigaction/19-4 \ sigaction/19-5 \ sigaction/19-6 \ sigaction/19-7 \ sigaction/19-8 \ sigaction/19-9 \ sigaction/23-10 \ sigaction/23-11 \ sigaction/23-12 \ sigaction/23-13 \ sigaction/23-14 \ sigaction/23-15 \ sigaction/23-16 \ sigaction/23-17 \ sigaction/23-18 \ sigaction/23-19 \ sigaction/23-1 \ sigaction/23-20 \ sigaction/23-21 \ sigaction/23-22 \ sigaction/23-23 \ sigaction/23-24 \ sigaction/23-25 \ sigaction/23-26 \ sigaction/23-2 \ sigaction/23-3 \ sigaction/23-4 \ sigaction/23-5 \ sigaction/23-6 \ sigaction/23-7 \ sigaction/23-8 \ sigaction/23-9 \ sigaction/28-10 \ sigaction/28-11 \ sigaction/28-12 \ sigaction/28-13 \ sigaction/28-14 \ sigaction/28-15 \ sigaction/28-16 \ sigaction/28-17 \ sigaction/28-18 \ sigaction/28-19 \ sigaction/28-1 \ sigaction/28-20 \ sigaction/28-21 \ sigaction/28-22 \ sigaction/28-23 \ sigaction/28-24 \ sigaction/28-25 \ sigaction/28-26 \ sigaction/28-2 \ sigaction/28-3 \ sigaction/28-4 \ sigaction/28-5 \ sigaction/28-6 \ sigaction/28-7 \ sigaction/28-8 \ sigaction/28-9 \ sigaction/29-1 \ sigaction/30-1 \ sigaction/9-1

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

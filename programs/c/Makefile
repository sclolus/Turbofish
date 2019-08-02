BUILD_DIR = build
VPATH += src

# AS = i686-turbofish-ld
CC = i686-turbofish-gcc

SRC_C = Clone \
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

ifeq ($(GNU),yes)
CFLAGS += -Wall -Wextra -m32
CC = gcc
else
# LDFLAGS = -m elf_i386 -dynamic-linker /lib/ld-linux.so.2 /usr/lib32/crt1.o /usr/lib32/crti.o /usr/lib32/crtn.o -lc
CFLAGS = -Wall -Wextra
endif

#ASFLAGS = -f elf

ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

TARGET_C = $(addprefix $(BUILD_DIR)/, $(basename $(notdir $(SRC_C))))

.PHONY = all clean fclean re

all: $(TARGET_C)

build/%: %.c Makefile
	$(CC) $(CFLAGS) -o $@ $<

install:
	cp -v $(TARGET_C) ../../rust_kernel/src/userland/

clean:
	rm -f $(TARGET_C)

fclean: clean

re: fclean all

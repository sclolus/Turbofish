
ifeq ($(findstring ProcessGroup,$(MAKECMDGOALS)),ProcessGroup)
	TARGET = ProcessGroup
	SRC_C = ProcessGroup
endif


ifeq ($(findstring DeepThought,$(MAKECMDGOALS)),DeepThought)
	TARGET = DeepThought
	SRC_C = DeepThought
endif

ifeq ($(findstring Clone,$(MAKECMDGOALS)),Clone)
	TARGET = Clone
	SRC_C = Clone
endif

ifeq ($(findstring Ud2,$(MAKECMDGOALS)),Ud2)
	TARGET = Ud2
	SRC_C = Ud2
endif

ifeq ($(findstring Wait,$(MAKECMDGOALS)),Wait)
	TARGET = Wait
	SRC_C = Wait
endif

ifeq ($(findstring ConnectionlessSimpleTest,$(MAKECMDGOALS)),ConnectionlessSimpleTest)
	TARGET = ConnectionlessSimpleTest
	SRC_C = ConnectionlessSimpleTest
endif

ifeq ($(findstring ConnectionOrientedSimpleTest,$(MAKECMDGOALS)),ConnectionOrientedSimpleTest)
	TARGET = ConnectionOrientedSimpleTest
	SRC_C = ConnectionOrientedSimpleTest
endif

ifeq ($(findstring Csignal,$(MAKECMDGOALS)),Csignal)
	TARGET = Csignal
	SRC_C = Csignal
endif

ifeq ($(findstring Getuid,$(MAKECMDGOALS)),Getuid)
	TARGET = Getuid
	SRC_C = Getuid
endif

ifeq ($(findstring Kill,$(MAKECMDGOALS)),Kill)
	TARGET = Kill
	SRC_C = Kill
endif

ifeq ($(findstring RecursiveSignal,$(MAKECMDGOALS)),RecursiveSignal)
	TARGET = RecursiveSignal
	SRC_C = RecursiveSignal
endif

ifeq ($(findstring SegFault,$(MAKECMDGOALS)),SegFault)
	TARGET = SegFault
	SRC_C = SegFault
endif

ifeq ($(findstring SonSendSignal,$(MAKECMDGOALS)),SonSendSignal)
	TARGET = SonSendSignal
	SRC_C = SonSendSignal
endif

ifeq ($(findstring Timer,$(MAKECMDGOALS)),Timer)
	TARGET = Timer
	SRC_C = Timer
endif

ifeq ($(findstring SaRestart,$(MAKECMDGOALS)),SaRestart)
	TARGET = SaRestart
	SRC_C = SaRestart
endif

ifeq ($(findstring SaRestartMultiple,$(MAKECMDGOALS)),SaRestartMultiple)
	TARGET = SaRestartMultiple
	SRC_C = SaRestartMultiple
endif

ifeq ($(findstring NoSaRestart,$(MAKECMDGOALS)),NoSaRestart)
	TARGET = NoSaRestart
	SRC_C = NoSaRestart
endif

ifeq ($(findstring NoSaRestartMultiple,$(MAKECMDGOALS)),NoSaRestartMultiple)
	TARGET = NoSaRestartMultiple
	SRC_C = NoSaRestartMultiple
endif

ifeq ($(findstring Continue,$(MAKECMDGOALS)),Continue)
	TARGET = Continue
	SRC_C = Continue
endif

ifeq ($(findstring ContinueHardcore,$(MAKECMDGOALS)),ContinueHardcore)
	TARGET = ContinueHardcore
	SRC_C = ContinueHardcore
endif

ifeq ($(findstring DummyRead,$(MAKECMDGOALS)),DummyRead)
	TARGET = DummyRead
	SRC_C = DummyRead
endif

ifeq ($(findstring SignalSimple,$(MAKECMDGOALS)),SignalSimple)
    TARGET = SignalSimple
    SRC_C = SignalSimple
endif

ifeq ($(findstring SignalSimpleDuo,$(MAKECMDGOALS)),SignalSimpleDuo)
    TARGET = SignalSimpleDuo
    SRC_C = SignalSimpleDuo
endif

ifeq ($(findstring SignalSimpleDuoRecurse,$(MAKECMDGOALS)),SignalSimpleDuoRecurse)
    TARGET = SignalSimpleDuoRecurse
    SRC_C = SignalSimpleDuoRecurse
endif

ifeq ($(findstring SignalSimpleStopContinue,$(MAKECMDGOALS)),SignalSimpleStopContinue)
    TARGET = SignalSimpleStopContinue
    SRC_C = SignalSimpleStopContinue
endif

ifeq ($(findstring SignalStopContinueOverload,$(MAKECMDGOALS)),SignalStopContinueOverload)
    TARGET = SignalStopContinueOverload
    SRC_C = SignalStopContinueOverload
endif

ifeq ($(findstring GetArgs,$(MAKECMDGOALS)),GetArgs)
    TARGET = GetArgs
    SRC_C = GetArgs
endif

# --- Preserve this above part. It is stricly system ---
ifeq ($(GNU),yes)
SRC_ASM +=
else
SRC_ASM += start
endif

BUILD_DIR = build
VPATH += src

AS = nasm
CC = gcc

ifeq ($(GNU),yes)
CFLAGS += -m32
# Add GNU libc includes
LDFLAGS = -m elf_i386 -dynamic-linker /lib/ld-linux.so.2 /usr/lib32/crt1.o /usr/lib32/crti.o /usr/lib32/crtn.o -lc
else
CFLAGS += -m32 -fno-builtin -fno-stack-protector -nostdlib -nodefaultlibs -fno-exceptions -nostdinc
LDFLAGS = -T linker.ld -m elf_i386
endif
ASFLAGS = -f elf
CFLAGS += -m32
ifeq ($(LDMAP),yes)
    LDFLAGS += -M
endif

ifeq ($(GNU),yes)
	IFLAGS +=
else
	IFLAGS += -Ilibc/include
endif

OBJ_DIR = obj
OBJ_ASM = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_ASM)))))
OBJ_C = $(addprefix $(OBJ_DIR)/, $(addsuffix .o, $(basename $(notdir $(SRC_C)))))

.PHONY = all clean fclean re

all: $(TARGET)
# Do nothing while A specific target is not specified

$(TARGET): $(OBJ_ASM) $(OBJ_C)
ifeq ($(GNU),yes)
	$(LD) $(LDFLAGS) -o $(BUILD_DIR)/$@ $^
else
	make -C libc all DEBUG=$(DEBUG) OPTIM=$(OPTIM)
	$(LD) $(LDFLAGS) -o $(BUILD_DIR)/$@ $^ libc/libc.a
endif

$(OBJ_DIR)/%.o: %.asm Makefile $(HEADERS)
	$(AS) $(ASFLAGS) -o $@ $<

$(OBJ_DIR)/%.o: %.c Makefile $(HEADERS)
	$(CC) -c $(CFLAGS) $(IFLAGS) -o $@ $<

clean:
	find $(OBJ_DIR) -type f -name '*.o' -delete
	make -C libc clean DEBUG=$(DEBUG) OPTIM=$(OPTIM)

fclean:
	find $(OBJ_DIR) -type f -name '*.o' -delete
	make -C libc fclean DEBUG=$(DEBUG) OPTIM=$(OPTIM)
	rm -f $(BUILD_DIR)/[^.]*

re: fclean all

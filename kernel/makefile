OBJ=kernel.img
CC= gcc -Wall -Wextra -m32 -fno-builtin -c
ASM=nasm -f elf

all: $(OBJ) 

MAIN=kernel.o
GRAPH=char.o draw.o
SYSTEM=gdt.o idt.o irq.o irq_keyboard.o pic.o vesa.o energie.o
CMD=lspci.o io.o

OBJDIR=./
SYSDIR=./system/
GRAPHDIR=./graphic/
KERNELDIR=./base/
CMDDIR=./cmd/

kernel.img: $(MAIN) $(SYSTEM) $(GRAPH) $(CMD)
	cd $(OBJDIR)
	ld --ignore-unresolved-symbol _GLOBAL_OFFSET_TABLE_ -melf_i386 --oformat binary -Ttext 1000 $^ -o $@

kernel.o: $(KERNELDIR)kernel.c
	$(CC) $(KERNELDIR)kernel.c -o $(OBJDIR)kernel.o 

char.o: $(GRAPHDIR)char.asm
	$(ASM) $(GRAPHDIR)char.asm -o $(OBJDIR)char.o

draw.o: $(GRAPHDIR)draw.asm
	$(ASM) $(GRAPHDIR)draw.asm -o $(OBJDIR)draw.o

gdt.o: $(SYSDIR)gdt.c
	$(CC) $(SYSDIR)gdt.c -o $(OBJDIR)gdt.o

idt.o: $(SYSDIR)idt.c
	$(CC) $(SYSDIR)idt.c -o $(OBJDIR)idt.o

vesa.o: $(SYSDIR)vesa.asm
	$(ASM) $(SYSDIR)vesa.asm -o $(OBJDIR)vesa.o

irq.o: $(SYSDIR)irq.asm
	$(ASM) $(SYSDIR)irq.asm -o $(OBJDIR)irq.o

irq_keyboard.o: $(SYSDIR)irq_keyboard.asm
	$(ASM) $(SYSDIR)irq_keyboard.asm -o $(OBJDIR)irq_keyboard.o

pic.o: $(SYSDIR)pic.asm
	$(ASM) $(SYSDIR)pic.asm -o $(OBJDIR)pic.o

energie.o: $(SYSDIR)energie.asm
	$(ASM) $(SYSDIR)energie.asm -o $(OBJDIR)energie.o

lspci.o: $(CMDDIR)lspci.asm
	$(ASM) $(CMDDIR)lspci.asm -o $(OBJDIR)lspci.o

io.o: $(CMDDIR)io.asm
	$(ASM) $(CMDDIR)io.asm -o $(OBJDIR)io.o

#################################
# int.o: int.asm		#
#	$(ASM) -o $@ $^		#
#	nasm -f elf -o $@ $^ 	#
#.o: .c 			#
#	$(CC) -c $^		#
#################################

clean:
	rm -f $(OBJ) *.o
	
fclean:
	rm -f $(OBJ) *.o
	rm -f kernel.img


OBJ=floppyA

all: $(OBJ) 

floppyA: bootsect kernel
	cat bootsect kernel /dev/zero | dd of=floppyA bs=512 count=2880

kernel: kernel.o screen.o
	ld --ignore-unresolved-symbol _GLOBAL_OFFSET_TABLE_  -melf_i386 --oformat binary -Ttext 1000 kernel.o screen.o -o kernel

#kernel.o: kernel.asm 
#nasm -f elf -o $@ $^

bootsect: bootsect.asm
	nasm -f bin -o $@ $^

%.o: %.c 
	gcc -m32 -c $^

clean:
	rm -f $(OBJ) *.o bootsect kernel

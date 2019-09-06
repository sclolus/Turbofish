export TURBOFISH_ROOT := $(shell pwd)
include $(TURBOFISH_ROOT)/boilerplates.mk

RAM_AMOUNT = 128
IMG_DISK = image_disk.img
IMAGE_SIZE = 65536
LOOP_DEVICE = $(shell sudo losetup -f)
KERNEL_DIRECTORY = $(KERNEL)_kernel

.PHONY: system

all: system_root $(IMG_DISK)
# compile and install libc
	make -C libc

	make -C programs
	cp pulp_fiction.txt $(SYSTEM_ROOT)/home

# This below sub-make should be integrated
#	make -C dash
#	make -C coreutils

# it could be interesting to find a way for preventing kernel relinking (or shell rust program)
	make -C $(KERNEL_DIRECTORY) DEBUG=$(DEBUG) OPTIM=$(OPTIM)
	cp -vf $(KERNEL_DIRECTORY)/build/kernel.elf $(SYSTEM_ROOT)/turbofish

	sudo losetup -fP $(IMG_DISK)
	sudo mount $(LOOP_DEVICE)p1 /mnt

# synchronize all modifieds files
	@echo ""
	@echo "### Syncing files ###"
	sudo rsync -av $(SYSTEM_ROOT)/ /mnt/
	@echo ""

	sudo umount /mnt
	sudo losetup -d $(LOOP_DEVICE)

# build system root image directory
system_root:
	mkdir -pv $(SYSTEM_ROOT)
	mkdir -pv $(SYSTEM_ROOT)/bin
	mkdir -pv $(SYSTEM_ROOT)/dev
	mkdir -pv $(SYSTEM_ROOT)/etc
	mkdir -pv $(SYSTEM_ROOT)/grub
	mkdir -pv $(SYSTEM_ROOT)/home
	mkdir -pv $(SYSTEM_ROOT)/turbofish

$(IMG_DISK):
	dd if=/dev/zero of=$(IMG_DISK) bs=1024 count=$(IMAGE_SIZE)
	echo -e "o\nn\np\n1\n2048\n\na\nw\n" | sudo fdisk $(IMG_DISK)
	sudo losetup -fP $(IMG_DISK)
	sudo mkfs.ext2 $(LOOP_DEVICE)p1
	sudo mount $(LOOP_DEVICE)p1 /mnt
	echo "(hd0) " $(LOOP_DEVICE) > loopdevice.map

# test - This module provides the "test" command which is used to evaluate an expression.
# echo - This module provides the "echo" command.
# vga - This module provides VGA support.
# normal - This module provides "Normal Mode" which is the opposite of "Rescue Mode".
# elf - This module loads ELF files.
# multiboot - multiboot - This module provides various functions needed to support multi-booting systems.
# part_msdos - This module provides support for MS-DOS (MBR) partitions and partitioning tables.
# ext2 - This module provides support for EXT2 filesystems.
# sleep - This module allow to sleep a while.
	sudo grub-install --target=i386-pc --no-floppy --grub-mkdevicemap=loopdevice.map --install-modules="sleep test echo vga normal elf multiboot part_msdos ext2" --locales="" --fonts="" --themes=no --modules="part_msdos" --boot-directory=/mnt $(LOOP_DEVICE) -v

	sudo cp -vf grub/grub.cfg /mnt/grub
# synchronize disk with our system root directory
	sudo rsync -av /mnt/ $(SYSTEM_ROOT)/
	sudo umount /mnt
	sudo losetup -d $(LOOP_DEVICE)

clean:
# This below sub-make should be integrated
#	make -C dash clean
#	make -C coreutils clean
	make -C programs clean
	make -C libc clean
	make -C $(KERNEL_DIRECTORY) clean

fclean:
# This below sub-make should be integrated
#	make -C dash fclean
#	make -C coreutils fclean
	make -C programs fclean
	make -C libc fclean
	make -C $(KERNEL_DIRECTORY) fclean

	sudo rm -rvf system
	rm -vf loopdevice.map
	rm -vf $(IMG_DISK)

mrproper:
	find . -name "*~" -exec rm -v {} \;
	find . -name "*#" -exec rm -v {} \;
	find . -name "*.orig" -exec rm -v {} \;

re: clean all

copy: $(IMG_DISK)
	dd if=$(IMG_DISK) of=/dev/sdb bs=1024 count=$(IMAGE_SIZE)
	sync

exec:
	qemu-system-x86_64 -m $(RAM_AMOUNT) -vga std -enable-kvm -cpu IvyBridge -drive format=raw,file=$(IMG_DISK)

exec_serial_port:
	qemu-system-x86_64 -m $(RAM_AMOUNT) -vga std -enable-kvm -cpu IvyBridge -drive format=raw,file=$(IMG_DISK) -device isa-debug-exit,iobase=0xf4,iosize=0x04 --serial stdio

exec_sata:
	qemu-system-x86_64 -m $(RAM_AMOUNT) -vga std -enable-kvm -cpu IvyBridge \
	-drive file=$(IMG_DISK),if=none,id=toto,format=raw \
	-device ich9-ahci,id=ahci \
	-device ide-drive,drive=toto,bus=ahci.0 \

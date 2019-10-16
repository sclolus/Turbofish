export TURBOFISH_ROOT := $(shell pwd)
include $(TURBOFISH_ROOT)/boilerplates.mk

RAM_AMOUNT = 512
IMG_DISK = image_disk.img
IMAGE_SIZE = 524288
FIRST_PART_SIZE = $$(($(IMAGE_SIZE) - 1024 * 10))
LOOP_DEVICE = $(shell sudo losetup -f)
KERNEL_DIRECTORY = $(KERNEL)_kernel

.PHONY: system

all: system_root $(IMG_DISK)
# compile and install libc
	make -C libc

	make -C programs

# This below sub-make should be integrated
#	make -C dash
#	make -C coreutils

# it could be interesting to find a way for preventing kernel relinking (or shell rust program)
	make -C $(KERNEL_DIRECTORY)
	sudo cp -vf $(KERNEL_DIRECTORY)/build/kernel.elf $(SYSTEM_ROOT)/turbofish

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
	sudo mkdir -pv $(SYSTEM_ROOT)
	sudo mkdir -pv $(SYSTEM_ROOT)/bin
	sudo mkdir -pv $(SYSTEM_ROOT)/bin/wolf3D
	sudo mkdir -pv $(SYSTEM_ROOT)/dev
	sudo mkdir -pv $(SYSTEM_ROOT)/etc
	sudo mkdir -pv $(SYSTEM_ROOT)/var
	sudo mkdir -pv $(SYSTEM_ROOT)/grub
	sudo mkdir -pv $(SYSTEM_ROOT)/home
	sudo mkdir -pv $(SYSTEM_ROOT)/home/$(STANDARD_USER)
	sudo cp files/shinit -pv $(SYSTEM_ROOT)/home/$(STANDARD_USER)/.shinit
	sudo cp files/pulp_fiction.txt $(SYSTEM_ROOT)/home/$(STANDARD_USER)
	sudo cp common/medias/univers.bmp $(SYSTEM_ROOT)/home
	sudo cp common/medias/wanggle.bmp $(SYSTEM_ROOT)/home
	sudo cp common/medias/asterix.bmp $(SYSTEM_ROOT)/home
	sudo chown -R 1000:1000 $(SYSTEM_ROOT)/home/$(STANDARD_USER)
	sudo mkdir -pv $(SYSTEM_ROOT)/turbofish
	sudo mkdir -pv $(SYSTEM_ROOT)/turbofish/mod
	sudo mkdir -pv $(SYSTEM_ROOT)/root
	sudo cp files/shinit -v $(SYSTEM_ROOT)/root/.shinit
	sudo chmod 0700 $(SYSTEM_ROOT)/root

$(IMG_DISK):
	dd if=/dev/urandom of=$(IMG_DISK) bs=1024 count=$(IMAGE_SIZE)
	echo -e "o\nn\np\n1\n2048\n$(FIRST_PART_SIZE)\na\nw\n" | sudo fdisk $(IMG_DISK)
	echo -e "n\np\n2\n\n\nw\n" | sudo fdisk $(IMG_DISK)
	sudo losetup -fP $(IMG_DISK)
	sudo mkfs.ext2 $(LOOP_DEVICE)p1
	sudo mkfs.ext2 $(LOOP_DEVICE)p2
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

	sudo rm -rvf $(SYSTEM_ROOT)
	rm -rvf build_coreutils
	rm -rvf build_dash
	rm -rvf build_procps
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
	qemu-system-x86_64 -m $(RAM_AMOUNT) -vga std -enable-kvm -cpu IvyBridge -drive format=raw,file=$(IMG_DISK) -rtc base=localtime,clock=rt,driftfix=none

exec_serial_port:
	qemu-system-x86_64 -m $(RAM_AMOUNT) -vga std -enable-kvm -cpu IvyBridge -drive format=raw,file=$(IMG_DISK) -device isa-debug-exit,iobase=0xf4,iosize=0x04 --serial stdio

exec_sata:
	qemu-system-x86_64 -m $(RAM_AMOUNT) -vga std -enable-kvm -cpu IvyBridge \
	-drive file=$(IMG_DISK),if=none,id=toto,format=raw \
	-device ich9-ahci,id=ahci \
	-device ide-drive,drive=toto,bus=ahci.0 \

# Quick fix for regenerate Bindgen
bindgen:
	make -C libc re
	make -C libc install
	cd rust_kernel/dependencies/libc_binding && rm src/libc.rs && make && cargo build

unix:
	./install_dash.sh
	./install_coreutils.sh
	./install_procps.sh
	make

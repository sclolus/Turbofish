IMG_DISK = image_disk.img
IMAGE_SIZE = 8192

all: $(IMG_DISK)
	make -C kernel
	sudo losetup -fP $(IMG_DISK)
	sudo mount /dev/loop0p1 /mnt
	sudo cp -vf kernel/kernel.elf /mnt
	sudo umount /mnt
	sudo losetup -d /dev/loop0

$(IMG_DISK):
	dd if=/dev/zero of=$(IMG_DISK) bs=1024 count=$(IMAGE_SIZE)
	( echo -e "o\nn\np\n1\n2048\n\nw\n") | sudo fdisk $(IMG_DISK)
	sudo losetup -fP $(IMG_DISK)
	sudo mkfs.ext2 /dev/loop0p1
	sudo mount /dev/loop0p1 /mnt
	echo "(hd0) /dev/loop0" > loop0device.map
	sudo grub-install --no-floppy --grub-mkdevicemap=loop0device.map --locales="fr" --fonts="en_US" --themes=no --modules="part_msdos part_gpt" --boot-directory=/mnt /dev/loop0 -v
	sudo cp -vf grub/grub.cfg /mnt/grub 
	sudo umount /mnt
	sudo losetup -d /dev/loop0

clean:
	make -C kernel fclean

fclean:
	make -C kernel fclean
	rm -vf loop0device.map
	rm -vf $(IMG_DISK)

re: clean all

copy: $(IMG_DISK)
	sudo dd if=$(IMG_DISK) of=/dev/sdb bs=1024 count=$(IMAGE_SIZE)
	sync

exec:
	qemu-system-x86_64 -m 32 -vga std -hda $(IMG_DISK) -enable-kvm

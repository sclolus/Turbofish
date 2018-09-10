IMG_DISK = image_disk.img

all: $(IMG_DISK)
	make -C kernel
	sudo losetup -fP $(IMG_DISK)
	sudo mount /dev/loop0p1 /mnt
	sudo cp -vf kernel/kernel.elf /mnt
	sudo umount /mnt
	sudo losetup -d /dev/loop0

$(IMG_DISK):
	dd if=/dev/zero of=$(IMG_DISK) bs=512 count=32768
	( echo -e "o\nn\np\n1\n\n\nw\n") | sudo fdisk $(IMG_DISK)
	sudo losetup -fP $(IMG_DISK)
	sudo mkfs.ext4 /dev/loop0p1
	sudo mount /dev/loop0p1 /mnt
	echo "(hd0) /dev/loop0" > loop0device.map
	sudo grub-install --no-floppy --grub-mkdevicemap=loop0device.map --modules="part_msdos" --boot-directory=/mnt /dev/loop0 -v
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

exec:
	qemu-system-x86_64 -m 64 -vga std -hda $(IMG_DISK) -enable-kvm

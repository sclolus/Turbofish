IMG_DISK = image_disk.img

all: $(IMG_DISK) boot_sector.bin
	dd if=boot_sector.bin of=$(IMG_DISK) bs=1 count=440 conv=notrunc

$(IMG_DISK):
	dd if=/dev/zero of=$(IMG_DISK) bs=512 count=8192
	( echo -e "o\nn\np\n1\n2048\n\nw\n") | sudo fdisk $(IMG_DISK)
	sudo losetup -fP $(IMG_DISK)
	sudo mkfs.ext2 /dev/loop0p1
	sudo losetup -d /dev/loop0

boot_sector.bin: boot_sector.asm
	nasm -f bin boot_sector.asm -o boot_sector.bin

clean:
	rm -f boot_sector.bin

fclean: clean
	rm -f $(IMG_DISK)

re: fclean all

exec:
	qemu-system-x86_64 -hda $(IMG_DISK) --enable-kvm -m 128
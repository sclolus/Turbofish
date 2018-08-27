NAME = data.img

all: $(NAME)

$(NAME): boot_sector.img kernel.img
	cat boot/bootSector.img kernel/kernel.img /dev/zero | dd of=$(NAME) bs=512 count=144

boot_sector.img:
	make -C boot

kernel.img:
	make -C kernel

clean:
	make -C boot clean
	make -C kernel clean

fclean:
	make -C boot fclean
	make -C kernel fclean
	rm -f $(NAME)

re: fclean all

exec:
	qemu-system-x86_64 -fda $(NAME)

NAME = data.raw

all: $(NAME)

$(NAME): boot_sector.img init_sequence.img kernel.img
	cat boot/boot_sector/boot_sector.img boot/init_sequence/init_sequence.img kernel/kernel.img /dev/zero | dd of=$(NAME) bs=512 count=2000

boot_sector.img:
	make -C boot/boot_sector
	
init_sequence.img:
	make -C boot/init_sequence

kernel.img:
	make -C kernel

clean:
	make -C boot/boot_sector clean
	make -C boot/init_sequence clean
	make -C kernel clean

fclean:
	make -C boot/boot_sector fclean
	make -C boot/init_sequence fclean
	make -C kernel fclean
	rm -f $(NAME)

re: fclean all

exec:
	qemu-system-x86_64 --vga std -fda $(NAME)

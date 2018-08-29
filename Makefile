NAME = data.img

all: $(NAME)

$(NAME): boot_sector.img alpha.img kernel.img
	cat boot/boot_sector.img fonts/alpha.img kernel/kernel.img /dev/zero | dd of=$(NAME) bs=512 count=144

boot_sector.img:
	make -C boot

kernel.img:
	make -C kernel

alpha.img:
	make -C fonts

clean:
	rm -f polices/alpha.img
	make -C boot clean
	make -C kernel clean
	make -C fonts clean

fclean:
	rm -f polices/alpha.img
	make -C boot fclean
	make -C kernel fclean
	make -C fonts fclean
	rm -f $(NAME)

re: fclean all

exec:
	qemu-system-x86_64 -fda $(NAME)

NAME = boot_sector.img

all: $(NAME)

$(NAME): boot_sector.asm
	nasm -f bin boot_sector.asm -o $(NAME)

clean:
	rm -f $(NAME)

fclean: clean

re: fclean all
NAME = boot_sector.img

all: $(NAME)

$(NAME): bootsect.asm
	nasm -f bin bootsect.asm -o $(NAME)

clean:
	rm -f $(NAME)

fclean: clean

re: fclean all
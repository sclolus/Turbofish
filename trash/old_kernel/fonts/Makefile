NAME = alpha.img

all: $(NAME)

$(NAME): alpha.asm
	nasm -f bin alpha.asm -o $(NAME)

clean:
	rm -f $(NAME)
	
fclean: clean

re: fclean all
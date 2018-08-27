NAME = init_sequence.img

all: $(NAME)

$(NAME): init_sequence.asm
	nasm -f bin init_sequence.asm -o $(NAME)

clean:
	rm -f $(NAME)

fclean: clean

re: fclean all
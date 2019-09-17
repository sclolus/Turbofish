# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

all:
	make -C dummy
	make -C key

fclean:
	make -C dummy fclean
	make -C key fclean

clean: fclean

re: fclean all

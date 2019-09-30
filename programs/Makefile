# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

all:
	make -C c
	make -C tests
	make -C system
	make -C su
	make -C modules

fclean:
	make fclean -C c
	make fclean -C tests
	make fclean -C system
	make fclean -C su
	make fclean -C modules

clean: fclean

re: fclean all

# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

all:
	make -C dummy_module

fclean:
	make -C dummy_module fclean

clean: fclean

re: fclean all

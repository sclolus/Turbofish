# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

all:
	make -C charles
	make -C charly

fclean:
	make fclean -C charles
	make fclean -C charly

clean: fclean

re: fclean all

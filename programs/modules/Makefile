# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

all:
	make -C dummy
	make -C key
	make -C rtc

fclean:
	make -C dummy fclean
	make -C key fclean
	make -C rtc fclean

clean: fclean

re: fclean all

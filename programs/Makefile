USERLAND = ../rust_kernel/src/userland/
TARGET_DIR = $(USERLAND)/bin
all:
	mkdir -pv $(USERLAND)
	mkdir -pv $(USERLAND)/etc
	mkdir -pv $(TARGET_DIR)

	make -C ./c/
	make install -C c

	make -C tests
	make install -C tests

	make -C system
	make install -C system

	make -C shell
	make install -C shell
	cp shell/build/shell $(TARGET_DIR)

	make -C su
	make install -C su


# make -C ./rust/fork_bomb
# cp ./rust/fork_bomb/fork_bomb $(TARGET_DIR)
# make -C ./rust/fork_fucker
# cp ./rust/fork_fucker/fork_fucker $(TARGET_DIR)
# make -C ./rust/fork_me_baby
# cp ./rust/fork_me_baby/fork_me_baby $(TARGET_DIR)
# make -C ./rust/mordak
# cp ./rust/mordak/mordak $(TARGET_DIR)
# make -C ./rust/prempt_me
# cp ./rust/prempt_me/prempt_me $(TARGET_DIR)
# make -C ./rust/richard
# cp ./rust/richard/richard $(TARGET_DIR)
# make -C ./rust/signal
# cp ./rust/signal/signal $(TARGET_DIR)
# make -C ./rust/sleepers
# cp ./rust/sleepers/sleepers $(TARGET_DIR)
# make -C ./rust/stack_overflow
# cp ./rust/stack_overflow/stack_overflow $(TARGET_DIR)
# make -C ./rust/sys_stack_overflow
# cp ./rust/sys_stack_overflow/sys_stack_overflow $(TARGET_DIR)

fclean:
	make fclean -C c
	make fclean -C system
	make fclean -C shell
	make fclean -C su

re: fclean all

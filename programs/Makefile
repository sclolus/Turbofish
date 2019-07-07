TARGET_DIR = ../rust_kernel/src/userland/
all:
	make -C ./c/ Clone
	make -C ./c/ Ud2
	make -C ./c/ Wait
	make -C ./c/ ConnectionlessSimpleTest
	make -C ./c/ ConnectionOrientedSimpleTest
	make -C ./c/ Csignal
	make -C ./c/ Getuid
	make -C ./c/ Kill
	make -C ./c/ RecursiveSignal
	make -C ./c/ SegFault
	make -C ./c/ SonSendSignal
	make -C ./c/ Timer
	make -C ./c/ SaRestart
	make -C ./c/ SaRestartMultiple
	make -C ./c/ NoSaRestart
	make -C ./c/ NoSaRestartMultiple
	make -C ./c/ Continue
	make -C ./c/ ContinueHardcore
	make -C ./c/ DummyRead
	make -C ./c/ SignalSimple
	make -C ./c/ SignalSimpleDuo
	make -C ./c/ SignalSimpleDuoRecurse
	make -C ./c/ SignalSimpleStopContinue
	make -C ./c/ SignalStopContinueOverload
	make -C ./c/ SignalSimpleStopContinue
	make -C ./c/ GetArgs
	cp ./c/build/* $(TARGET_DIR)

	make -C ./rust/fork_bomb
	cp ./rust/fork_bomb/fork_bomb $(TARGET_DIR)
	make -C ./rust/fork_fucker
	cp ./rust/fork_fucker/fork_fucker $(TARGET_DIR)
	make -C ./rust/fork_me_baby
	cp ./rust/fork_me_baby/fork_me_baby $(TARGET_DIR)
	make -C ./rust/mordak
	cp ./rust/mordak/mordak $(TARGET_DIR)
	make -C ./rust/prempt_me
	cp ./rust/prempt_me/prempt_me $(TARGET_DIR)
	make -C ./rust/richard
	cp ./rust/richard/richard $(TARGET_DIR)
	make -C ./rust/signal
	cp ./rust/signal/signal $(TARGET_DIR)
	make -C ./rust/sleepers
	cp ./rust/sleepers/sleepers $(TARGET_DIR)
	make -C ./rust/stack_overflow
	cp ./rust/stack_overflow/stack_overflow $(TARGET_DIR)
	make -C ./rust/sys_stack_overflow
	cp ./rust/sys_stack_overflow/sys_stack_overflow $(TARGET_DIR)

	make -C shell
	cp shell/build/shell $(TARGET_DIR)

fclean:
	rm c/build/*

re: fclean all

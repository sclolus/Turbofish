#include "sched.h"
#include "stdio.h"
#include "stdlib.h"

extern int errno;

extern int sys_clone(void *, int);

// inspired by the linux clone syscall
int	clone(int (*fn)(void *), void *child_stack,
		  int flags, void *arg/*, pid_t *ptid, void *newtls, pid_t *ctid*/) {

	// push the args on the child_stack
	int *new_child_stack = child_stack;
	new_child_stack--;
	*new_child_stack = (int)arg;
	new_child_stack--;
	*new_child_stack = flags;
	new_child_stack--;
	*new_child_stack = (int)child_stack;
	new_child_stack--;
	*new_child_stack = (int)fn;


	// here we don't use the user_syscall, as we must do a hack to
	// call continue_clone_child in the child
	int ret = sys_clone(new_child_stack, flags);

	if (ret < 0) {
		errno = -ret;
		return -1;
	}
	return ret;
}

// continue the clone fonction if we are in a child and the child_stack != NULL
int	continue_clone_child(int (*fn)(void *), void *child_stack, int flags, void *arg) {
	if (child_stack == NULL) {
		printf("panic child stack == NULL\n");
		exit(1);
	}
	(void)flags;
	if (fn != NULL) {
		exit(fn(arg));
	}
	printf("fn null\n");
	exit(1);
	return 1;
}

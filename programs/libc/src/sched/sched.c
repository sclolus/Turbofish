#include "sched.h"
#include "stdio.h"
#include "stdlib.h"

extern int errno;

extern int sys_clone(void *, int);

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


	int ret = sys_clone(new_child_stack, flags);

//_user_syscall(CLONE, 2, child_stack, flags);

	if (ret < 0) {
		errno = -ret;
		return -1;
	}
	if (ret  == 0) {
		printf("ret null\n");
		exit(1);
		}

	/* 
	 * } else if ((ret == 0) && (fn > 0)) {
	 * 	exit(fn(arg));
	 * }
	 */
	return ret;
}

// continue the clone fonction if we are in a child
int	continue_clone_child(int (*fn)(void *), void *child_stack, int flags, void *arg) {
	(void)child_stack;
	(void)flags;
	if (fn != NULL) {
		exit(fn(arg));
	}
	printf("fn null\n");
	exit(1);
	return 1;
}

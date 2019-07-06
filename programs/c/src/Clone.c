
#define _GNU_SOURCE
#include <sched.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

static int *PTR = NULL;

static int childFunc(void *arg) {
	printf("child func, arg: %s\n", (char *)arg);
	PTR = malloc(4);
	*PTR = 42;
	sleep(2);
	return 1;           /* Child terminates now */
}

#define STACK_SIZE (1024 * 1024)    /* Stack size for cloned child */

int main()
{
	char *stack;                    /* Start of stack buffer */
	char *stackTop;                 /* End of stack buffer */
	pid_t pid;

	/* Allocate stack for child */

	/* printf("%llX\n", childFunc); */
	stack = malloc(STACK_SIZE);
	if (stack == NULL) {
		dprintf(2, "malloc faild\n");
		exit(1);
	}
	stackTop = stack + STACK_SIZE;  /* Assume stack grows downward */

	/* pid = clone(childFunc, stackTop, CLONE_VM | CLONE_THREAD | SIGCHLD, NULL); */
	/* int a, b, c; */
	pid = clone(childFunc, stackTop, CLONE_VM | CLONE_THREAD | CLONE_SIGHAND, "abc"/*, &a, &b, &c*/);
	printf("after clone\n");
	if (pid == -1) {
		dprintf(2, "clone faild\n");
		/* perror("clone"); */
		exit(1);
	}
	sleep(1);
	printf("%d\n", *PTR);
	sleep(3);
	printf("prepare to exit\n");


	exit(0);
}


#include <stdio.h>

char **environ;

int errno;

void basic_constructor(int argc, char **argv, char **envp) {
	environ = envp;
	/* printf("** libc constructor called: argc: %i, argc: %p, envp: %p ***\n",
	       argc,
	       argv,
	       envp); */
	(void)argc;
	(void)argv;
	errno = 0;
}

void basic_destructor(void) {
	/* puts("*** libc destructor called ***"); */
}

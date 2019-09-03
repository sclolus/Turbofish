#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>

#define __PRIVATE_USE_ENV_H__
# include "./stdlib/env.h"
#undef __PRIVATE_USE_ENV_H__

char **environ;

int errno;

#define array_size __array_size

void basic_constructor(int argc, char **argv, char **envp) {
	if (!envp) {
		handle_null_environ();
		envp = environ;
	}

	size_t	env_size = array_size((void **)envp);

	char	**heap_env = malloc(sizeof(char*) * (env_size + 1));

	if (!heap_env) {
		dprintf(2, "*** Heap allocation of envp in clib constructor failed ***\n");
		exit(EXIT_FAILURE);
	}

	heap_env[env_size] = NULL;

	for (size_t i = 0; i < env_size; i++) {
		heap_env[i] = strdup(envp[i]);

		if (!heap_env[i]) {
			dprintf(2, "*** Heap allocation of some env entry in clib constructor failed ***\n");
			exit(EXIT_FAILURE);
		}
	}

	environ = heap_env;
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


#undef array_size

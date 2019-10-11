#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <assert.h>
#include <user_syscall.h>

#define __PRIVATE_USE_ENV_H__
# include "./stdlib/env.h"
#undef __PRIVATE_USE_ENV_H__

char **environ;

int errno;

struct kernel kernel;

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

	// Get informations about kernel
	int ret = _user_syscall(GET_KERNEL_PROPERTIES, 1, &kernel);
	if (ret < 0) {
		dprintf(2, "*** Cannot get kernel properties ***\n");
		exit(EXIT_FAILURE);
	}
	(void)argc;
	(void)argv;
	errno = 0;
}

void basic_destructor(void) {
	/* puts("*** libc destructor called ***"); */
}

extern unsigned char __init_array_start;
extern unsigned char __init_array_end;


void	call_init_array_ctors(void)
{
	void   (**current_ctor)(void) = (void *)&__init_array_start;
	void   (**current_ctor_end)(void) = (void *)&__init_array_end;

	/// Probably no init_array.
	if (!current_ctor) {
		return ;
	}
	while (current_ctor < current_ctor_end) {
		(*current_ctor)();
		current_ctor++;
	}
}
#undef array_size

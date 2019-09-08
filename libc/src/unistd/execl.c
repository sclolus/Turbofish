#include <ltrace.h>
#include <errno.h>
#include <unistd.h>
#include <stdarg.h>
#include <stdint.h>
#include <limits.h>
#include <string.h>
#include <stdlib.h>

// Necessary boilerplate since linux does not defined ARG_MAX in limits.h...
#ifndef ARG_MAX
# define ARG_MAX 4096 * 8
#endif

extern char **environ;

int          execl(const char *path, const char *arg0, ...)
{
	TRACE
	va_list	ap;
	char	**argv;

	argv = malloc(sizeof(char *) * 2);
	if (!argv) {
		return -1;
	}
	argv[0] = (char *)arg0;
	argv[1] = NULL;
	va_start(ap, arg0);

	size_t	env_bytes = 0;
	for (uint32_t i = 0; environ[i]; i++) {
		env_bytes += strlen(environ[i]);
	}

	size_t	total_bytes = strlen(path) + strlen(arg0) + env_bytes;

	if (total_bytes > ARG_MAX) {  // Check for maximum byte count in argument list reached.
		va_end(ap);
		free(argv);
		errno = E2BIG;
		return -1;
	}

	char	*current_arg = NULL;
	// Collect argv from variadic list.
	for (uint32_t i = 1; (current_arg = va_arg(ap, char *)); i++) {
		total_bytes += strlen(current_arg);

		if (total_bytes > ARG_MAX) { // Check for maximum byte count in argument list reached.
			va_end(ap);
			free(argv);
			errno = E2BIG;
			return -1;
		}

		char	**new_argv = realloc(argv, sizeof(char *) * (i + 1 + 1));

		if (!new_argv) {
			va_end(ap);
			free(argv);
			return -1;
		}
		argv = new_argv;
		argv[i] = current_arg;
		argv[i + 1] = NULL;
	}
	va_end(ap);

	int ret = execve(path, argv, environ); // Assumes execve set the correct errno value.

	free(argv); // Need to free argv on error.
	return ret;
}

#ifdef UNIT_TESTS
# include <criterion/criterion.h>

Test(execl, too_big_arg_fails_with_e2big) {
	char	*very_big_argument = malloc(ARG_MAX + 1);

	cr_assert(very_big_argument);
	very_big_argument[ARG_MAX] = '\0';
	memset(very_big_argument, 'a', ARG_MAX);

	int ret = execl("/bin/ls", "ls", very_big_argument, (char *)0x0);

	cr_assert_eq(ret, -1);
	cr_assert_eq(errno, E2BIG);
	free(very_big_argument);
}

Test(excl, too_big_args_fails_with_e2big) {
	const size_t base_arg_size = ARG_MAX / 2 + 1;

	char	*arg1 = malloc(base_arg_size + 1);
	char	*arg2 = malloc(base_arg_size + 1);

	cr_assert(arg1 && arg2);
	arg1[base_arg_size] = '\0';
	arg2[base_arg_size] = '\0';
	memset(arg1, 'a', base_arg_size);
	memset(arg2, 'b', base_arg_size);

	int ret = execl("/bin/ls", "ls", arg1, arg2, (char *)0x0);
	cr_assert_eq(ret, -1);
	cr_assert_eq(errno, E2BIG);
	free(arg1);
	free(arg2);
}

#endif /* UNIT_TESTS */

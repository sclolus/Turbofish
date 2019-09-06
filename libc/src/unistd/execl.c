#include <errno.h>
#include <unistd.h>
#include <stdarg.h>
#include <stdint.h>
#include <limits.h>
#include <string.h>
#include <stdlib.h>

extern char **environ;

int          execl(const char *path, const char *arg0, ...)
{
	va_list	ap;
	char	**argv;

	argv = malloc(sizeof(char *) * 2);
	if (!argv) {
		errno = ENOMEM; // Explicitly set ENOMEN, shall we do this ?
		return -1;
	}
	argv[0] = (char *)arg0;
	argv[1] = NULL;
	va_start(ap, arg0);
	// Collect argv from variadic list.
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
			errno = ENOMEM; // Explicitly set ENOMEN, shall we do this ?
			free(argv);
			return -1;
		}
		argv[i] = current_arg;
		argv[i + 1] = NULL;
	}
	va_end(ap);

	int ret = execve(path, argv, environ); // Assumes execve set the correct errno value.

	free(argv); // Need to free argv on error.
	return ret;
}

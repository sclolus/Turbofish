#include <unistd.h>
#include <errno.h>
#include <ltrace.h>
#include <limits.h>
#include <string.h>
#include <assert.h>
#include <stdlib.h>

extern char **environ;

/* There are two distinct ways in which the contents of the process image file may cause the execution to fail, distinguished by the setting of errno to either [ENOEXEC] or [EINVAL] (see the ERRORS section). In the cases where the other members of the exec family of functions would fail and set errno to [ENOEXEC], the execlp() and execvp() functions shall execute a command interpreter and the environment of the executed command shall be as if the process invoked the sh utility using execl() as follows: */

/* execl(<shell path>, arg0, file, arg1, ..., (char *)0); */

/* where <shell path> is an unspecified pathname for the sh utility, file is the process image file, and for execvp(), where arg0, arg1, and so on correspond to the values passed to execvp() in argv[0], argv[1], and so on. */

/* 	For those forms not containing an envp pointer (execl(), execv(), execlp(), and execvp()), the environment for the new process image shall be taken from the external variable environ in the calling process. */

#warning missing tests

int          execvp(const char *file, char *const argv[])
{
	TRACE
	int ret = execve(file, argv, environ);

	// execve shall never return if successful.
	assert(ret == -1);

	// Handle special case: If the file is not a binary executable,
	// assume it is a shell script, thereby executing it via '/bin/sh'.
	// (path is unspecified by posix).
	if (errno == ENOEXEC) {
		size_t argv_size = 0;

		for (size_t i = 0; argv[i]; i++) {
			argv_size++;
		}

		char	**new_argv = calloc(sizeof(char *) * (argv_size + 1 + 1), 1);

		if (!new_argv) {
			return -1;
		}

		new_argv[0] = argv[0];
		new_argv[1] = (char *)file;

		if (argv_size) {
			memcpy(new_argv + 2, argv + 1, argv_size - 1);
		}
		ret = execve("/bin/sh", new_argv, environ);
		free(new_argv);
	}
	return ret;
}

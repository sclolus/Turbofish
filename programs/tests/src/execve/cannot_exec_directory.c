#include <assert.h>
#include <errno.h>
#include <unistd.h>
#include <stdlib.h>
#include <wait.h>
#include <sys/stat.h>

extern char **environ;

int main(void)
{
	if (-1 == access("/bin", F_OK)) {
		assert(0 == mkdir("/bin/", 0777));
	}

	assert(-1 == execve("/bin/", (char *[]){"env", NULL}, environ));
	assert(errno == EACCESS);
	assert(-1 == execve("/bin", (char *[]){"env", NULL}, environ));
	assert(errno == EACCESS);
	return EXIT_SUCCESS;
}

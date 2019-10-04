#include <assert.h>
#include <errno.h>
#include <unistd.h>
#include <stdlib.h>
#include <wait.h>
#include <sys/stat.h>

extern char **environ;

int main(void)
{
	char	filename[256];
	pid_t	pid = getpid();

	snprintf(filename, sizeof(filename), "cannot_exec_dir_%u", pid);
	assert(-1 != mkdir(filename, 0777));


	assert(-1 == execve(filename, (char *[]){filename, NULL}, environ));
	assert(errno == EACCES);

	snprintf(filename, sizeof(filename), "cannot_exec_dir_%u/", pid);
	assert(-1 == execve(filename, (char *[]){filename, NULL}, environ));
	assert(errno == EACCES);
	assert(-1 != rmdir(filename));
	return EXIT_SUCCESS;
}

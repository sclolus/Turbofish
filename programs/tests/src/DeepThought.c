#include <stdlib.h>
#include <errno.h>
#include <unistd.h>
#include <stdio.h>
#include <wait.h>

#ifndef GNU
#include <custom.h>
#endif

struct program_test {
	char *path;
	/* char **argv; */
};

static struct program_test TEST_PROGRAMS[] = {
	{.path = "/bin/signal/SignalSimple"},
	{.path = "/bin/signal/SignalSimpleDuo"},
	{.path = "/bin/ProcessGroup"},
	{.path = "/bin/execve/argv"},
	{.path = "/bin/wait/wait"},
	{.path = "/bin/wait/wuntraced"},
	{.path = "/bin/mprotect/mprotect"},
	{.path = "/bin/mmap/mmap"},
	{.path = "/bin/atexit/atexit"},
	{.path = "/bin/munmap/munmap"},
	{.path = "/bin/sigprocmask/sigprocmask"},
	{.path = "/bin/isatty/isatty"},
	{.path = "/bin/pipe/pipe_fucker"},
	{.path = "/bin/pipe/pipe_fister"},
	{.path = "/bin/pipe/pipe_lorem_ipsum"},
	{.path = "/bin/math/roundf"},
	{.path = "/bin/ctype/longlong"},
	{.path = "/bin/lseek/sda"},
	{.path = "/bin/lseek/lseek_return"},
	{.path = "/bin/dirent/dummy_root"},
	{.path = "/bin/read/read_pulp_fiction"},
	{.path = "/bin/execl/execl"},
	{.path = "/bin/is_str_bullshit/is_str_bullshit"},
};

void _exit_qemu(int val)
{
#ifdef GNU
	exit(val);
#else
	exit_qemu(val);
#endif
}

#define TEST_PROGRAMS_LEN sizeof(TEST_PROGRAMS) / sizeof(struct program_test)

int main() {
	for (unsigned int i = 0; i < TEST_PROGRAMS_LEN; i++) {
		printf("executing %s\n", TEST_PROGRAMS[i].path);
		char *env[] = {NULL};
		pid_t pid = fork();
		if (pid < 0) {
			perror("fork failed");
			exit(1);
		} else if (pid == 0) {
			char *args[2] = {
				TEST_PROGRAMS[i].path,
				NULL,
			};
			pid_t child_pid = getpid();
			printf("child_pid: %d\n", child_pid);
			execve(TEST_PROGRAMS[i].path, args, env);
			perror("execve failed");
			_exit_qemu(1);
		} else {
			int status;
			int ret = wait(&status);
			if (ret == -1) {
				perror("Deepthought wait failed");
				_exit_qemu(1);
			}
			if (status != 0) {
				// qemu exit fail
				dprintf(2, "test: '%s' failed", TEST_PROGRAMS[i].path);
				dprintf(2, "status '%d'", status);
				sleep(1000);
				if (!WIFEXITED(status)) {
					_exit_qemu(1);
				}
				if (WEXITSTATUS(status) != 0) {
					_exit_qemu(1);
				}
			}
		}
	}
	/* sleep(100); */
	printf("All tests succesfull\n");
	sleep(5);
	_exit_qemu(0);
}

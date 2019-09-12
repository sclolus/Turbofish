#include <stdlib.h>
#include <errno.h>
#include <unistd.h>
#include <stdio.h>
#include <wait.h>

#ifndef GNU
#include <custom.h>
#endif

// Control if tests are launched one by one or not
bool LINEAR = false;

struct program_test {
	char *path;
	pid_t pid;
	/* char **argv; */
};

static struct program_test TEST_PROGRAMS[] = {
	{.path = "/bin/link/link"},
	{.path = "/bin/symlink/symlink"},
	{.path = "/bin/dir/mkdir"},
	{.path = "/bin/dir/mkdir_exist"},
	{.path = "/bin/dir/rmdir_not_empty"},
	{.path = "/bin/unlink/unlink_multiple"},
	{.path = "/bin/write/create_write_read"},
	{.path = "/bin/unlink/unlink"},
	{.path = "/bin/access/access"},
	{.path = "/bin/signal/SignalSimple"},
	{.path = "/bin/signal/SignalSimpleDuo"},
	{.path = "/bin/ProcessGroup"},
	{.path = "/bin/execve/argv"},
	{.path = "/bin/execve/cannot_exec_directory"},
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
	{.path = "/bin/umask/umask"},
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

size_t find_program(pid_t pid) {
	for (size_t i = 0; i < TEST_PROGRAMS_LEN; i++) {
		if (TEST_PROGRAMS[i].pid == pid) {
			return i;
		}
	}
	dprintf(2, "program not found WTF\n");
	_exit_qemu(1);
	return 1;
}

void launch_test(size_t i) {
	TEST_PROGRAMS[i].pid = -1;
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
		for (int j = 0; j < 10; j++) {
			printf("%s\n", TEST_PROGRAMS[i].path);
		}
		while (1) {}
		_exit_qemu(1);
	}
	//father
	TEST_PROGRAMS[i].pid = pid;
}

void wait_test() {
	int status;
	int ret = wait(&status);
	if (ret == -1) {
		perror("Deepthought wait failed");
		_exit_qemu(1);
	}
	if (status != 0) {
		// qemu exit fail
		size_t i = find_program(ret);
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

int main() {
	for (size_t i = 0; i < TEST_PROGRAMS_LEN; i++) {
		launch_test(i);
		if (LINEAR) {
			wait_test();
		}
	}
	if (!LINEAR) {
		for (size_t i = 0; i < TEST_PROGRAMS_LEN; i++) {
			wait_test();
		}
	}
	/* sleep(100); */
	printf("All tests succesfull\n");
	sleep(5);
	_exit_qemu(0);
}

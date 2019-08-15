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
	int status;
	/* char **argv; */
};
static struct program_test TEST_PROGRAMS[] = {
	{.path = "/bin/SignalSimple"},
	{.path = "/bin/SignalSimpleDuo"},
	{.path = "/bin/ProcessGroup"},
	{.path = "/bin/kill/1-1"},
	{.path = "/bin/kill/1-1"},
	{.path = "/bin/signal/1-1"},
	{.path = "/bin/signal/2-1"},
	{.path = "/bin/signal/3-1"},
	{.path = "/bin/signal/5-1"},
	{.path = "/bin/signal/6-1"},
	{.path = "/bin/signal/7-1"},
	{.path = "/bin/sigaction/1-1"},
	{.path = "/bin/sigaction/1-10"},
	{.path = "/bin/sigaction/1-11"},
	{.path = "/bin/sigaction/1-12"},
	{.path = "/bin/sigaction/1-13"},
	{.path = "/bin/sigaction/1-14"},
	{.path = "/bin/sigaction/1-15"},
	{.path = "/bin/sigaction/1-16"},
	{.path = "/bin/sigaction/1-17"},
	{.path = "/bin/sigaction/1-18"},
	{.path = "/bin/sigaction/1-19"},
	{.path = "/bin/sigaction/1-2"},
	{.path = "/bin/sigaction/1-20"},
	{.path = "/bin/sigaction/1-21"},
	{.path = "/bin/sigaction/1-22"},
	{.path = "/bin/sigaction/1-23"},
	{.path = "/bin/sigaction/1-24"},
	{.path = "/bin/sigaction/1-25"},
	{.path = "/bin/sigaction/1-26"},
	{.path = "/bin/sigaction/1-3"},
	{.path = "/bin/sigaction/1-4"},
	{.path = "/bin/sigaction/1-5"},
	{.path = "/bin/sigaction/1-6"},
	{.path = "/bin/sigaction/1-7"},
	{.path = "/bin/sigaction/1-8"},
	{.path = "/bin/sigaction/1-9"},
	{.path = "/bin/sigaction/2-1"},
	{.path = "/bin/sigaction/2-10"},
	{.path = "/bin/sigaction/2-11"},
	{.path = "/bin/sigaction/21-1"},
	{.path = "/bin/sigaction/2-12"},
	{.path = "/bin/sigaction/2-13"},
	{.path = "/bin/sigaction/2-14"},
	{.path = "/bin/sigaction/2-15"},
	{.path = "/bin/sigaction/2-16"},
	{.path = "/bin/sigaction/2-17"},
	{.path = "/bin/sigaction/2-18"},
	{.path = "/bin/sigaction/2-19"},
	{.path = "/bin/sigaction/2-2"},
	{.path = "/bin/sigaction/2-20"},
	{.path = "/bin/sigaction/2-21"},
	{.path = "/bin/sigaction/22-1"},
	{.path = "/bin/sigaction/22-10"},
	{.path = "/bin/sigaction/22-11"},
	{.path = "/bin/sigaction/22-12"},
	{.path = "/bin/sigaction/22-13"},
	{.path = "/bin/sigaction/22-14"},
	{.path = "/bin/sigaction/22-15"},
	{.path = "/bin/sigaction/22-16"},
	{.path = "/bin/sigaction/22-17"},
	{.path = "/bin/sigaction/22-18"},
	{.path = "/bin/sigaction/22-19"},
	{.path = "/bin/sigaction/2-22"},
	{.path = "/bin/sigaction/22-2"},
	{.path = "/bin/sigaction/22-20"},
	{.path = "/bin/sigaction/22-21"},
	{.path = "/bin/sigaction/22-22"},
	{.path = "/bin/sigaction/22-23"},
	{.path = "/bin/sigaction/22-24"},
	{.path = "/bin/sigaction/22-25"},
	{.path = "/bin/sigaction/22-26"},
	{.path = "/bin/sigaction/2-23"},
	{.path = "/bin/sigaction/22-3"},
	{.path = "/bin/sigaction/2-24"},
	{.path = "/bin/sigaction/22-4"},
	{.path = "/bin/sigaction/2-25"},
	{.path = "/bin/sigaction/22-5"},
	{.path = "/bin/sigaction/2-26"},
	{.path = "/bin/sigaction/22-6"},
	{.path = "/bin/sigaction/22-7"},
	{.path = "/bin/sigaction/22-8"},
	{.path = "/bin/sigaction/22-9"},
	{.path = "/bin/sigaction/2-3"},
	{.path = "/bin/sigaction/2-4"},
	{.path = "/bin/sigaction/2-5"},
	{.path = "/bin/sigaction/25-1"},
	{.path = "/bin/sigaction/25-10"},
	{.path = "/bin/sigaction/25-11"},
	{.path = "/bin/sigaction/25-12"},
	{.path = "/bin/sigaction/25-13"},
	{.path = "/bin/sigaction/25-14"},
	{.path = "/bin/sigaction/25-15"},
	{.path = "/bin/sigaction/25-16"},
	{.path = "/bin/sigaction/25-17"},
	{.path = "/bin/sigaction/25-18"},
	{.path = "/bin/sigaction/25-19"},
	{.path = "/bin/sigaction/25-2"},
	{.path = "/bin/sigaction/25-20"},
	{.path = "/bin/sigaction/25-21"},
	{.path = "/bin/sigaction/25-22"},
	{.path = "/bin/sigaction/25-23"},
	{.path = "/bin/sigaction/25-24"},
	{.path = "/bin/sigaction/25-25"},
	{.path = "/bin/sigaction/25-26"},
	{.path = "/bin/sigaction/25-3"},
	{.path = "/bin/sigaction/25-4"},
	{.path = "/bin/sigaction/25-5"},
	{.path = "/bin/sigaction/25-6"},
	{.path = "/bin/sigaction/25-7"},
	{.path = "/bin/sigaction/25-8"},
	{.path = "/bin/sigaction/25-9"},
	{.path = "/bin/sigaction/2-6"},
	{.path = "/bin/sigaction/2-7"},
	{.path = "/bin/sigaction/2-8"},
	{.path = "/bin/sigaction/2-9"},
	{.path = "/bin/sigaction/3-1"},
	{.path = "/bin/sigaction/3-10"},
	{.path = "/bin/sigaction/3-11"},
	{.path = "/bin/sigaction/3-12"},
	{.path = "/bin/sigaction/3-13"},
	{.path = "/bin/sigaction/3-14"},
	{.path = "/bin/sigaction/3-15"},
	{.path = "/bin/sigaction/3-16"},
	{.path = "/bin/sigaction/3-17"},
	{.path = "/bin/sigaction/3-18"},
	{.path = "/bin/sigaction/3-19"},
	{.path = "/bin/sigaction/3-2"},
	{.path = "/bin/sigaction/3-20"},
	{.path = "/bin/sigaction/3-21"},
	{.path = "/bin/sigaction/3-22"},
	{.path = "/bin/sigaction/3-23"},
	{.path = "/bin/sigaction/3-24"},
	{.path = "/bin/sigaction/3-25"},
	{.path = "/bin/sigaction/3-26"},
	{.path = "/bin/sigaction/3-3"},
	{.path = "/bin/sigaction/3-4"},
	{.path = "/bin/sigaction/3-5"},
	{.path = "/bin/sigaction/3-6"},
	{.path = "/bin/sigaction/3-7"},
	{.path = "/bin/sigaction/3-8"},
	{.path = "/bin/sigaction/3-9"},
	{.path = "/bin/sigaction/4-1"},
	{.path = "/bin/sigaction/4-10"},
	{.path = "/bin/sigaction/4-100"},
	{.path = "/bin/sigaction/4-101"},
	{.path = "/bin/sigaction/4-102"},
	{.path = "/bin/sigaction/4-103"},
	{.path = "/bin/sigaction/4-104"},
	{.path = "/bin/sigaction/4-11"},
	{.path = "/bin/sigaction/4-12"},
	{.path = "/bin/sigaction/4-13"},
	{.path = "/bin/sigaction/4-14"},
	{.path = "/bin/sigaction/4-15"},
	{.path = "/bin/sigaction/4-16"},
	{.path = "/bin/sigaction/4-17"},
	{.path = "/bin/sigaction/4-18"},
	{.path = "/bin/sigaction/4-19"},
	{.path = "/bin/sigaction/4-2"},
	{.path = "/bin/sigaction/4-20"},
	{.path = "/bin/sigaction/4-21"},
	{.path = "/bin/sigaction/4-22"},
	{.path = "/bin/sigaction/4-23"},
	{.path = "/bin/sigaction/4-24"},
	{.path = "/bin/sigaction/4-25"},
	{.path = "/bin/sigaction/4-26"},
	{.path = "/bin/sigaction/4-27"},
	{.path = "/bin/sigaction/4-28"},
	{.path = "/bin/sigaction/4-29"},
	{.path = "/bin/sigaction/4-3"},
	{.path = "/bin/sigaction/4-30"},
	{.path = "/bin/sigaction/4-31"},
	{.path = "/bin/sigaction/4-32"},
	{.path = "/bin/sigaction/4-33"},
	{.path = "/bin/sigaction/4-34"},
	{.path = "/bin/sigaction/4-35"},
	{.path = "/bin/sigaction/4-36"},
	{.path = "/bin/sigaction/4-37"},
	{.path = "/bin/sigaction/4-38"},
	{.path = "/bin/sigaction/4-39"},
	{.path = "/bin/sigaction/4-4"},
	{.path = "/bin/sigaction/4-40"},
	{.path = "/bin/sigaction/4-41"},
	{.path = "/bin/sigaction/4-42"},
	{.path = "/bin/sigaction/4-43"},
	{.path = "/bin/sigaction/4-44"},
	{.path = "/bin/sigaction/4-45"},
	{.path = "/bin/sigaction/4-46"},
	{.path = "/bin/sigaction/4-47"},
	{.path = "/bin/sigaction/4-48"},
	{.path = "/bin/sigaction/4-49"},
	{.path = "/bin/sigaction/4-5"},
	{.path = "/bin/sigaction/4-50"},
	{.path = "/bin/sigaction/4-51"},
	{.path = "/bin/sigaction/4-52"},
	{.path = "/bin/sigaction/4-53"},
	{.path = "/bin/sigaction/4-54"},
	{.path = "/bin/sigaction/4-55"},
	{.path = "/bin/sigaction/4-56"},
	{.path = "/bin/sigaction/4-57"},
	{.path = "/bin/sigaction/4-58"},
	{.path = "/bin/sigaction/4-59"},
	{.path = "/bin/sigaction/4-6"},
	{.path = "/bin/sigaction/4-60"},
	{.path = "/bin/sigaction/4-61"},
	{.path = "/bin/sigaction/4-62"},
	{.path = "/bin/sigaction/4-63"},
	{.path = "/bin/sigaction/4-64"},
	{.path = "/bin/sigaction/4-65"},
	{.path = "/bin/sigaction/4-66"},
	{.path = "/bin/sigaction/4-67"},
	{.path = "/bin/sigaction/4-68"},
	{.path = "/bin/sigaction/4-69"},
	{.path = "/bin/sigaction/4-7"},
	{.path = "/bin/sigaction/4-70"},
	{.path = "/bin/sigaction/4-71"},
	{.path = "/bin/sigaction/4-72"},
	{.path = "/bin/sigaction/4-73"},
	{.path = "/bin/sigaction/4-74"},
	{.path = "/bin/sigaction/4-75"},
	{.path = "/bin/sigaction/4-76"},
	{.path = "/bin/sigaction/4-77"},
	{.path = "/bin/sigaction/4-78"},
	{.path = "/bin/sigaction/4-79"},
	{.path = "/bin/sigaction/4-8"},
	{.path = "/bin/sigaction/4-80"},
	{.path = "/bin/sigaction/4-81"},
	{.path = "/bin/sigaction/4-82"},
	{.path = "/bin/sigaction/4-83"},
	{.path = "/bin/sigaction/4-84"},
	{.path = "/bin/sigaction/4-85"},
	{.path = "/bin/sigaction/4-86"},
	{.path = "/bin/sigaction/4-87"},
	{.path = "/bin/sigaction/4-88"},
	{.path = "/bin/sigaction/4-89"},
	{.path = "/bin/sigaction/4-9"},
	{.path = "/bin/sigaction/4-90"},
	{.path = "/bin/sigaction/4-91"},
	{.path = "/bin/sigaction/4-92"},
	{.path = "/bin/sigaction/4-93"},
	{.path = "/bin/sigaction/4-94"},
	{.path = "/bin/sigaction/4-95"},
	{.path = "/bin/sigaction/4-96"},
	{.path = "/bin/sigaction/4-97"},
	{.path = "/bin/sigaction/4-98"},
	{.path = "/bin/sigaction/4-99"},
	{.path = "/bin/sigaction/6-1"},
	{.path = "/bin/sigaction/6-10"},
	{.path = "/bin/sigaction/6-11"},
	{.path = "/bin/sigaction/6-12"},
	{.path = "/bin/sigaction/6-13"},
	{.path = "/bin/sigaction/6-14"},
	{.path = "/bin/sigaction/6-15"},
	{.path = "/bin/sigaction/6-16"},
	{.path = "/bin/sigaction/6-17"},
	{.path = "/bin/sigaction/6-18"},
	{.path = "/bin/sigaction/6-19"},
	{.path = "/bin/sigaction/6-2"},
	{.path = "/bin/sigaction/6-20"},
	{.path = "/bin/sigaction/6-21"},
	{.path = "/bin/sigaction/6-22"},
	{.path = "/bin/sigaction/6-23"},
	{.path = "/bin/sigaction/6-24"},
	{.path = "/bin/sigaction/6-25"},
	{.path = "/bin/sigaction/6-26"},
	{.path = "/bin/sigaction/6-3"},
	{.path = "/bin/sigaction/6-4"},
	{.path = "/bin/sigaction/6-5"},
	{.path = "/bin/sigaction/6-6"},
	{.path = "/bin/sigaction/6-7"},
	{.path = "/bin/sigaction/6-8"},
	{.path = "/bin/sigaction/6-9"},
	{.path = "/bin/sigaction/8-1"},
	{.path = "/bin/sigaction/8-10"},
	{.path = "/bin/sigaction/8-11"},
	{.path = "/bin/sigaction/8-12"},
	{.path = "/bin/sigaction/8-13"},
	{.path = "/bin/sigaction/8-14"},
	{.path = "/bin/sigaction/8-15"},
	{.path = "/bin/sigaction/8-16"},
	{.path = "/bin/sigaction/8-17"},
	{.path = "/bin/sigaction/8-18"},
	{.path = "/bin/sigaction/8-19"},
	{.path = "/bin/sigaction/8-2"},
	{.path = "/bin/sigaction/8-20"},
	{.path = "/bin/sigaction/8-21"},
	{.path = "/bin/sigaction/8-22"},
	{.path = "/bin/sigaction/8-23"},
	{.path = "/bin/sigaction/8-24"},
	{.path = "/bin/sigaction/8-25"},
	{.path = "/bin/sigaction/8-26"},
	{.path = "/bin/sigaction/8-3"},
	{.path = "/bin/sigaction/8-4"},
	{.path = "/bin/sigaction/8-5"},
	{.path = "/bin/sigaction/8-6"},
	{.path = "/bin/sigaction/8-7"},
	{.path = "/bin/sigaction/8-8"},
	{.path = "/bin/sigaction/8-9"},
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
			pid_t father_pid = getpid();
			printf("father_pid: %d\n", father_pid);
			printf("I am the father, i wait my child\n");
			int status;
			int ret = wait(&status);
			if (ret == -1) {
				perror("wait failed");
				_exit_qemu(1);
			}
			TEST_PROGRAMS[i].status = status;

		}
	}
	int failed = 0;
	for (unsigned int i = 0; i < TEST_PROGRAMS_LEN; i++) {

		int status = TEST_PROGRAMS[i].status;
		if (status != 0) {
			failed = 1;
			printf("test: '%s' failed", TEST_PROGRAMS[i].path);
			if (WIFEXITED(status)) {
				printf("status '%d'", WEXITSTATUS(status));
			}
		}
	}
	if (failed != 0) {
		_exit_qemu(1);
	}
	_exit_qemu(0);
}

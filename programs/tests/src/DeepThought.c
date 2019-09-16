#include <stdlib.h>
#include <errno.h>
#include <unistd.h>
#include <stdio.h>
#include <wait.h>
#include <stdbool.h>
#include <assert.h>
#include <fcntl.h>
#include <string.h>
#include <tools/tools.h>
#include <stdint.h>

// Command Sequence Introducer
#define CSI "\x1b["

// Command Sequnce Terminator
#define CST "m"

// Standard Color Introducer
#define SCI "38;5;"
#define STANDARD_COLOR(x) CSI SCI #x CST

# define BLACK STANDARD_COLOR(0)
# define RED STANDARD_COLOR(1)
# define GREEN STANDARD_COLOR(2)
# define YELLOW STANDARD_COLOR(3)
# define BLUE STANDARD_COLOR(4)
# define MAGENTA STANDARD_COLOR(5)
# define CYAN STANDARD_COLOR(6)
# define WHITE STANDARD_COLOR(7)
/* #define WHITE CSI SCI "7" CST */
/* #define RED CSI SCI "1" CST */
/* #define GREEN CSI SCI "1" CST */

// Control if tests are launched one by one or not
bool LINEAR = true;
bool EXIT_ON_FAILURE = false;
bool IMM_PRINT_FAILURE = true;

struct deepthought_info {
	char	*logging_dir;
	char	*failing_dir;
	uint8_t	exit_on_failure : 1,
		print_failure_immediately : 1;
};

struct deepthought_info g_deepthought_info = {
	.logging_dir = NULL,
	.failing_dir = NULL,
};

struct program_test {
	char	*path;
	char	*logging_dir;
	char	*basename;
	pid_t	pid;
	int	status;
	uint8_t	failed : 1;

	/* char **argv; */
};

static struct program_test TEST_PROGRAMS[] = {
	{.path = "/bin/fifo/fifo"},
	{.path = "/bin/execve/bad_elf"},
	{.path = "/bin/unlink/unlink_orphan"},
	{.path = "/bin/open/open_fails_with_eaccess_basic"},
	{.path = "/bin/rename/rename_dir_not_empty"},
	{.path = "/bin/rename/rename_dir_exist"},
	{.path = "/bin/rename/rename_file_exist"},
	{.path = "/bin/rename/rename"},
	{.path = "/bin/rename/rename_dir"},
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
	{.path = "/bin/math/pow"},
	{.path = "/bin/ctype/longlong"},
	{.path = "/bin/lseek/sda"},
	{.path = "/bin/lseek/lseek_return"},
	{.path = "/bin/dirent/dummy_root"},
	{.path = "/bin/read/read_pulp_fiction"},
	{.path = "/bin/execl/execl"},
	{.path = "/bin/is_str_bullshit/is_str_bullshit"},
	{.path = "/bin/umask/umask"},
	{.path = "/bin/statfs/statfs"},
	{.path = "/bin/fstatfs/fstatfs"},
	{.path = "/bin/statvfs/statvfs"},
	{.path = "/bin/fstatvfs/fstatvfs"},
	{.path = "/bin/chmod_tests/einval_mode"},
	{.path = "/bin/chmod_tests/chmod_normal"},
	{.path = "/bin/fchmod/einval_mode"},
	{.path = "/bin/fchmod/fchmod_normal"},
	{.path = "/bin/utime/utime_basic"},
	{.path = "/bin/chown_tests/chown_basic"},
	{.path = "/bin/fchown/fchown_basic"},
};

#define TEST_PROGRAMS_LEN sizeof(TEST_PROGRAMS) / sizeof(struct program_test)

// Some dummy implementation of the basename function, returns a pointer to the filename in `path`.
static char	*basename(char *path)
{
	char *filename = strrchr(path, '/');

	if (!filename) {
		filename = path;
	} else {
		filename += 1;
	}
	return filename;
}

size_t find_program(pid_t pid) {
	for (size_t i = 0; i < TEST_PROGRAMS_LEN; i++) {
		if (TEST_PROGRAMS[i].pid == pid) {
			return i;
		}
	}
	dprintf(2, "program not found WTF\n");
	return 1;
}

// Redirects the `fd` into the logging file `into`
void	redirect_into_logging_file(int fd, char *into, size_t test_id)
{
	char	filename[256 * 2];
	char	*dir = TEST_PROGRAMS[test_id].logging_dir;

	snprintf(filename, sizeof(filename), "%s/%s", dir, into);


	int redirect_fd = open(filename, O_CREAT | O_EXCL | O_RDWR, 0777);
	assert(redirect_fd != -1);
	printf(GREEN "Created logging file for %s: %s\n" WHITE, into, filename);
	// dup2 does not seems to work...
	/* assert(redirect_fd == dup2(fd, redirect_fd)); */
	close(fd);
	assert(fd == dup(redirect_fd));
}

void launch_test(size_t i) {
	TEST_PROGRAMS[i].pid = -1;
	printf(CYAN "executing %s\n" WHITE, TEST_PROGRAMS[i].path);
	char *test_name = basename(TEST_PROGRAMS[i].path);

	assert(test_name);
	char	*env[] = { NULL };
	pid_t	pid = fork();

	if (pid < 0) {
		perror("fork failed");
		exit(1);
	} else if (pid == 0) {
		char *args[2] = {
			TEST_PROGRAMS[i].path,
			NULL,
		};

		// Redirect stdout and stderr into their respective logging files.
		redirect_into_logging_file(STDERR_FILENO, "stderr", i);
		redirect_into_logging_file(STDOUT_FILENO, "stdout", i);

		pid_t child_pid = getpid();
		/* printf("child_pid: %d\n", child_pid); */
		execve(TEST_PROGRAMS[i].path, args, env);
		perror("execve failed");
		for (int j = 0; j < 10; j++) {
			printf("%s\n", TEST_PROGRAMS[i].path);
		}
		exit(1);
	}
	//father
	TEST_PROGRAMS[i].pid = pid;
}

static inline void  print_failed_test(size_t test_index)
{
	int status = TEST_PROGRAMS[test_index].status;

	dprintf(2, RED "=== test: '%s' failed -> status '%d' ===\n" WHITE, TEST_PROGRAMS[test_index].path, status);
}

void wait_test() {
	int status;
	int ret = wait(&status);
	size_t i = find_program(ret);

	if (ret == -1) {
		perror("Deepthought wait failed");
		exit(1);
	}
	TEST_PROGRAMS[i].status = status;

	if (status != 0) {
		// qemu exit fail
		TEST_PROGRAMS[i].failed = true;

		if (g_deepthought_info.print_failure_immediately) {
			print_failed_test(i);
		}

		char	linkname[256 * 2];
		char	target[256 * 2];

		snprintf(linkname, sizeof(linkname), "%s/%s", g_deepthought_info.failing_dir, TEST_PROGRAMS[i].basename);
		snprintf(target, sizeof(target), "../%s", TEST_PROGRAMS[i].basename);
		assert(0 == symlink(target, linkname));
		if (!WIFEXITED(status) && g_deepthought_info.exit_on_failure) {
			exit(1);
		}
		if (WEXITSTATUS(status) != 0 && g_deepthought_info.exit_on_failure) {
			exit(1);
		}
	} else {
		TEST_PROGRAMS[i].failed = false;
	}
}

# define LOGGING_DIR "test_logs"
# define LAST_LOGGING_DIR "last"

static void	build_logging_directory(void)
{
	char	dir_filename[256];
	char	failing_dir_filename[256 * 2];
	pid_t	pid = getpid();

	snprintf(dir_filename, sizeof(dir_filename), LOGGING_DIR "_%u", pid);
	snprintf(failing_dir_filename, sizeof(failing_dir_filename), "%s/failing", dir_filename);

	// Attempts to remove a possibly already existing LAST_LOGGING_DIR symlink
	unlink(LAST_LOGGING_DIR);
	if (0 != symlink(dir_filename, LAST_LOGGING_DIR)) {
		err_errno("Failed to symlink %s -> %s", LAST_LOGGING_DIR, dir_filename);
	}

	g_deepthought_info.logging_dir = strdup(dir_filename);
	assert(g_deepthought_info.logging_dir);

	g_deepthought_info.failing_dir = strdup(failing_dir_filename);
	assert(g_deepthought_info.failing_dir);


	assert(-1 != mkdir(dir_filename, 0777));
	assert(-1 != mkdir(failing_dir_filename, 0777));

	for (size_t i = 0; i < TEST_PROGRAMS_LEN; i++) {
		char	*test_dir_name = basename(TEST_PROGRAMS[i].path);
		assert(test_dir_name);
		char	basename[256];

		// putting the test_number in because name duplications and I'm lazy
		snprintf(basename, sizeof(basename), "%s_%lu", test_dir_name, i);
		test_dir_name = strdup(basename);
		assert(test_dir_name);
		TEST_PROGRAMS[i].basename = test_dir_name;

		char	filename[256 * 2];
		snprintf(filename, sizeof(filename), "%s/%s", dir_filename, test_dir_name);
		if (-1 == mkdir(filename, 0777)) {
			err_errno("Failed to creat logging directory %s", filename)
		}

		char	*dup = strdup(filename);

		assert(dup);
		TEST_PROGRAMS[i].logging_dir = dup;
	}
}

int main() {
	g_deepthought_info.exit_on_failure = EXIT_ON_FAILURE;
	g_deepthought_info.print_failure_immediately = IMM_PRINT_FAILURE;

	build_logging_directory();
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

	bool	all_success = true;
	for (size_t i = 0; i < TEST_PROGRAMS_LEN; i++) {
		if (TEST_PROGRAMS[i].failed) {
			all_success = false;
			print_failed_test(i);
		}
	}

	if (all_success) {
		printf(GREEN "All tests succesfull\n" WHITE);
	}
	return 0;
}

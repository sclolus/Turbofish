#include <string.h>
#include <errno.h>

static const char *signal_str[] = {
	"Unknown signal 0",
	"Hangup",
	"Interrupt",
	"Quit",
	"Illegal instruction",
	"Trace/breakpoint trap",
	"Aborted",
	"Bus error",
	"Floating point exception",
	"Killed",
	"User defined signal 1",
	"Segmentation fault",
	"User defined signal 2",
	"Broken pipe",
	"Alarm clock",
	"Terminated",
	"Stack fault",
	"Child exited",
	"Continued",
	"Stopped (signal)",
	"Stopped",
	"Stopped (tty input)",
	"Stopped (tty output)",
	"Urgent I/O condition",
	"CPU time limit exceeded",
	"File size limit exceeded",
	"Virtual timer expired",
	"Profiling timer expired",
	"Window changed",
	"I/O possible",
	"Power failure",
	"Bad system call",
};

char *strsignal(int signum) {
	if (signum < 0 || signum > 31) {
		errno = EINVAL;
		return "Unknown signal";
	}
	return (char *)signal_str[signum];
}

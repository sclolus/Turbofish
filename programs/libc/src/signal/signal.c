
#include "signal.h"
#include "string.h"

extern int user_sigaction(int signum, const struct sigaction *act, struct sigaction *oldact);
extern int errno;

/*
 * sigaction, rt_sigaction - examine and change a signal action
 */
int sigaction(int signum, const struct sigaction *act, struct sigaction *oldact)
{
	int ret = user_sigaction(signum, act, oldact);

	/*
	 * sigaction() returns 0 on success; on error, -1 is returned,
	 * and errno is set to indicate the error.
	 */
	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		return 0;
	}
}

#include "stdio.h"

/*
 * signal - ANSI C signal handling
 */
sighandler_t signal(int signum, sighandler_t handler)
{
	struct sigaction sig;

	printf("size of the big struct %lu\n", sizeof(struct sigaction));
	printf("size of struct %lu\n", sizeof(sigset_t));

	ft_memset(&sig, 0, sizeof(struct sigaction));
	sig.sa_handler = handler;

	int ret = sigaction(signum, &sig, NULL);
	/*
	 * signal() returns the previous value of the signal handler, or SIG_ERR on error.
	 * In the event of an error, errno is set to indicate the cause.
	 */
	if (ret < 0) {
		return (sighandler_t)-1;
	} else {
		// TODO: What is the previous value of the signal handler ?
		return handler;
	}
}

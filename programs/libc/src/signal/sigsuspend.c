#include <signal.h>
#include <errno.h>
#include <user_syscall.h>

	/*
	 * sigset_t oldmask;
	 *
	 * sigprocmask(sig_setmask, sigmask, &oldmask);
	 * int ret = pause();
	 * sigprocmask(sig_setmask, oldmask, null);
	 */
int sigsuspend(const sigset_t *sigmask) {
	int ret = _user_syscall(SIGSUSPEND, 1, sigmask);

	if (ret < 0) {
		errno = -ret;
		return -1;
	} else {
		errno = 0;
		return 0;
	}
}

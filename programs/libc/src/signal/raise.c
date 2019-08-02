
#include <unistd.h>
#include <signal.h>

/*
 * send a signal to the caller
 */
int raise(int sig) {
	/*
	 * raise() returns 0 on success, and nonzero for failure
	 */
	return kill(getpid(), sig);
}

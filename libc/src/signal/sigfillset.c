#include <ltrace.h>
#include <signal.h>

/*
 * sigfillset() initializes set to full, including all signals.
 */
int    sigfillset(sigset_t *set)
{
	TRACE
	*set = ~0;
	return 0;
}

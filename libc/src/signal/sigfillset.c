#include <signal.h>

/*
 * sigfillset() initializes set to full, including all signals.
 */
int    sigfillset(sigset_t *set)
{
	*set = ~0;
	return 0;
}

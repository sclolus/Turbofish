#include <ltrace.h>
#include <signal.h>

/*
 * sigemptyset() initializes the signal set given by set to empty,
 * with all signals excluded from the set.
 */
int    sigemptyset(sigset_t *set)
{
	TRACE
	*set = 0;
	return 0;
}

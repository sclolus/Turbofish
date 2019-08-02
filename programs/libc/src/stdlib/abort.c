#include <stdlib.h>
#include <signal.h>

/*
 * abort - cause abnormal process termination
 */
void abort(void)
{
	raise(SIGABRT);
	while (1) {}
}

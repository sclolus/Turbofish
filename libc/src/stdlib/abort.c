#include <ltrace.h>
#include <stdlib.h>
#include <signal.h>

/*
 * abort - cause abnormal process termination
 */
void abort(void)
{
	TRACE
	raise(SIGABRT);
	while (1) {}
}

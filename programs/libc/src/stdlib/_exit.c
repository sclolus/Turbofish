
#include <stdlib.h>

/* The _Exit() [CX] [Option Start]  and _exit() [Option End] functions shall not call functions registered with atexit() nor any registered signal handlers. [CX] [Option Start]  Open streams shall not be flushed. [Option End]  Whether open streams are closed (without flushing) is implementation-defined. Finally, the calling process shall be terminated with the consequences described below. */

// TODO: we just call exit for the moment
void _exit(int status)
{
	/*
	 * The exit() function does not return.
	 */
	exit(status);
}

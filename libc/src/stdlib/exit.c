#include <stdlib.h>
#include <unistd.h>

/*
 * exit - cause normal process termination
 */
/// The exit() function shall first call all functions registered by
/// atexit(), in the reverse order of their registration, except that
/// a function is called after any previously registered functions
/// that had already been called at the time it was registered. Each
/// function is called as many times as it was registered. If, during
/// the call to any such function, a call to the longjmp() function is
/// made that would terminate the call to the registered function, the
/// behavior is undefined.
/// 
/// If a function registered by a call to atexit() fails to return,
/// the remaining registered functions shall not be called and the
/// rest of the exit() processing shall not be completed. If exit() is
/// called more than once, the behavior is undefined.
/// 
/// The exit() function shall then flush all open streams with
/// unwritten buffered data and close all open streams. Finally, the
/// process shall be terminated [CX] [Option Start] with the same
/// consequences as described in Consequences of Process
/// Termination. [Option End]
void exit(int status)
{
	execute_atexit_handlers();
	_exit(status);
}

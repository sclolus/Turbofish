#include <stdlib.h>

#define ATEXIT_MAX 64

static void (*ATEXIT_FUNCS[ATEXIT_MAX])(void);

static size_t ATEXIT_FUNCS_INDEX = 0;

/// The atexit() function shall register the function pointed to by
/// func, to be called without arguments at normal program
/// termination. At normal program termination, all functions
/// registered by the atexit() function shall be called, in the
/// reverse order of their registration, except that a function is
/// called after any previously registered functions that had already
/// been called at the time it was registered. Normal termination
/// occurs either by a call to exit() or a return from main().
/// 
/// At least 32 functions can be registered with atexit().
/// The application should call sysconf() to obtain the value of
/// {ATEXIT_MAX}, the number of functions that can be
/// registered. There is no way for an application to tell how many
/// functions have already been registered with atexit().
///
/// Upon successful completion, atexit() shall return 0; otherwise, it
/// shall return a non-zero value.
int atexit(void(*f)(void))
{

	if (ATEXIT_FUNCS_INDEX < ATEXIT_MAX) {
		ATEXIT_FUNCS[ATEXIT_FUNCS_INDEX] = f;
		ATEXIT_FUNCS_INDEX++;
		return 0;
	}
	return -1;
	
}

void execute_atexit_handlers()
{
	while(1) {
		if (ATEXIT_FUNCS_INDEX == 0) {
			return ;
		}
		ATEXIT_FUNCS_INDEX--;
		ATEXIT_FUNCS[ATEXIT_FUNCS_INDEX]();
	}
}


#include "unistd.h"

extern void user_exit(int status);

/*
 * exit - cause normal process termination
 */
void exit(int status)
{
	/*
	 * The exit() function does not return.
	 */
	user_exit(status);
}

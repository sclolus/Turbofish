
#include "unistd.h"

extern uid_t user_getuid(void);

/*
 * getuid - get user identity
 */
uid_t getuid(void)
{
	/*
	 * This function is always successful.
	 */
	return user_getuid();
}

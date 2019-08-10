#include <unistd.h>

// The getegid() function shall return the effective group ID of the
// calling process. The getegid() function shall not modify errno.

#warning NOT IMPLEMENTED

gid_t getegid(void)
{
	return 42;
}

#include <ltrace.h>
#include <custom.h>
#include <user_syscall.h>
#include <errno.h>
#include <stdbool.h>

bool is_ptr_valid(const char *path)
{
	TRACE
	int ret = _user_syscall(IS_STR_VALID, 1, path);
	if (ret < 0) {
		errno = -ret;
		return false;
	} else {
		return true;
	}
}


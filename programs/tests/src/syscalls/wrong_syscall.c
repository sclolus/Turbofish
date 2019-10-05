#include <assert.h>
#include <user_syscall.h>
#include <unistd.h>
#include <errno.h>
#include <stdio.h>

int main(void)
{
	ssize_t ret = _user_syscall(4242, 0);

	assert(ret == -ENOSYS);
	return EXIT_SUCCESS;
}


#include <unistd.h>

extern int errno;

int main()
{
	ssize_t ret;
	char buf[128];

	while ((ret = read(0, buf, 128)) > 0) {
		write(1, buf, (size_t)ret);
	}
	if (ret < 0) {
		return -errno;
	} else {
		return 0;
	}
}

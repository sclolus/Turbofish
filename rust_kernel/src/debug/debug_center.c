
#include "i386_type.h"
#include "libft.h"

extern int get_cpu_features(void);

int write(int fd, char *buf, size_t len) {
	(void)fd;
	(void)buf;
	(void)len;
	return 0;
}

int debug_center(void) {
	printk("Les carotes sont cuites\n");
	return get_cpu_features();
}

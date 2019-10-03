#include <stdio.h>
#include <stdlib.h>
#include <sys/mount.h>

int main(int ac, char **av) {
	if (ac != 2) {
		dprintf(2, "usage: umount source \n     umount source charactere device of file\n");
		exit(1);
	}
	int ret = umount(av[1]);
	if (ret == -1) {
		perror("umount");
	}
	return 0;
}

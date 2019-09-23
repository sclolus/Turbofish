#include <stdio.h>
#include <stdlib.h>
#include <sys/mount.h>

int main(int ac, char **av) {
	if (ac != 3) {
		dprintf(2, "usage: mount source target\n   mount source charactere device of file on directory pointed by target\n");
		exit(1);
	}
	int ret = mount(av[1], av[2], "ext2", 0, NULL);
	if (ret == -1) {
		perror("mount");
	}
	return 0;
}

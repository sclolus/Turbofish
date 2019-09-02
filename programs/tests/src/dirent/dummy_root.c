#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>

int main(void)
{
	DIR *dir = opendir("/");
	if (dir == NULL) {
		perror("opendir");
		exit(1);
	}
	struct dirent *dirent;
	while ((dirent = readdir(dir)) != NULL) {
		printf("-> %s\n", dirent->d_name);
	}
	if (closedir(dir) < 0) {
		perror("closedir");
		exit(1);
	}
	return 0;
}

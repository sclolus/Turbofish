#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <mod.h>

int main(int argc, char *argv[])
{
	if (argc != 2) {
		dprintf(STDERR_FILENO, "usage: %s module_name\n", argv[0]);
		exit(1);
	}
	return insmod(argv[1]);
}

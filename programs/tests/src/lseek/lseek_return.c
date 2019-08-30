#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>

int main() {
	off_t ret = lseek(42, 0, SEEK_CUR);
	/*
	 * lseek(42, 42, SEEK_CUR);
	 * lseek(42, 4294967296, SEEK_CUR);
	 */
	if (!(ret == -1)) {
		dprintf(2, "bad return of lseek %lld\n", ret);
		exit(1);
	}
}

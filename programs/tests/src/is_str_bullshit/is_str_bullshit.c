#include <unistd.h>
#include <sys/mman.h>
#include <stdio.h>
#include <custom.h>
#include <stdlib.h>
#include <string.h>

int main() {
	int  page_size = getpagesize();
	char *addr = mmap(NULL, 256 * page_size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
	if (addr == MAP_FAILED) {
		perror("mmap");
		exit(1);
	}
	char *addr2 = mmap(NULL, 256 * page_size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
	if (addr2 == MAP_FAILED) {
		perror("mmap");
		exit(1);
	}

	if (addr2 != addr + 256 * page_size) {
		dprintf(2, "test requierement is false");
		exit(0);
	}
	int ret = munmap(addr2, 256 * page_size);
	addr = addr + 255 * page_size;
	if (ret == -1) {
		perror("munmap");
		exit(1);
	}
	memset(addr, 'a', page_size);

	if (is_ptr_valid(addr)) {
		dprintf(2, "ptr should not be valid, 1");
		exit(1);
	}
	perror("is_ptr_valid");
	if (is_ptr_valid(addr + page_size - 1)) {
		dprintf(2, "ptr should not be valid, 2");
		exit(1);
	}
	perror("is_ptr_valid");
	if (is_ptr_valid(addr + page_size)) {
		dprintf(2, "ptr should not be valid, 3");
		exit(1);
	}
	perror("is_ptr_valid");
	addr[page_size - 1] = '\0';
	if (!is_ptr_valid(addr)) {
		dprintf(2, "ptr should be valid");
		exit(1);
	}
}

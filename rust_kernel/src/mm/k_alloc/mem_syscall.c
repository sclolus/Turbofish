
#include "main_headers.h"

#define MAP_FAILED 0xffffffff

extern void *kmmap(size_t size);
int kmunmap(void *virt_addr);

/*
** Claim pages from Kernel, size may be calibrated to page_size.
*/

void		*get_new_pages(size_t size)
{
	void *new_page;
/*
	new_page = mmap(
		NULL,
		size,
		PROT_READ | PROT_WRITE,
		MAP_ANON | MAP_PRIVATE,
		-1,
		0);
*/
	new_page = kmmap(size);
	return (u32)new_page == MAP_FAILED ? NULL : new_page;
}

/*
** Say to lernel to destroy pages, size may be calibrated to page_size.
*/

int		destroy_pages(void *addr, size_t size)
{
/*
	return (munmap(
		addr,
		size));
*/
	(void)size;
	return (kmunmap(addr));
}

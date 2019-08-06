
#include "main_headers.h"

void *get_kernel_pages(size_t size);
int free_kernel_pages(void *addr, size_t size);

/*
** Claim pages from Kernel, size may be calibrated to page_size.
*/

void		*get_new_pages(size_t size)
{
	void *new_page;

	new_page = get_kernel_pages(size);

	return new_page == MAP_FAILED ? NULL : new_page;
}

/*
** Say to lernel to destroy pages, size may be calibrated to page_size.
*/

int		destroy_pages(void *addr, size_t size)
{
	return free_kernel_pages(addr, size);
}

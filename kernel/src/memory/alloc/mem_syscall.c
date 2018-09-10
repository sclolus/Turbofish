/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mem_syscall.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2018/04/22 17:44:07 by bmickael          #+#    #+#             */
/*   Updated: 2018/04/22 17:50:30 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "memory.h"

/*
** Claim pages from Kernel, size may be calibrated to page_size.
*/

void		*get_new_pages(size_t size)
{
	void *new_page;

	size_t _size = (size >> 12) + ((size & 0xFFF)  ? 1 : 0);

/*
	new_page = mmap(
		NULL,
		size,
		PROT_READ | PROT_WRITE,
		MAP_ANON | MAP_PRIVATE,
		-1,
		0);
*/
	new_page = alloc_frames(_size);
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
	return (free_frames(addr));
}

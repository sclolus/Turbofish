
#ifndef __PAGING_H__
# define __PAGING_H__

#include "i386_type.h"

void	init_frames(void);
void	*alloc_frames(u32 page_request);
int	free_frames(void *addr);
u32	count_frames(void);

#endif

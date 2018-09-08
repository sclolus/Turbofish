
#ifndef __PAGING_H__
# define __PAGING_H__

#include "i386_type.h"

void	init_frames(void);
void	*alloc_frames(u32 page_request);
int	free_frames(void *addr);
u32	count_frames(void);

u32	paginate(u32 directory, u32 segment, u32 page_request, u32 address);
int	unpaginate(u32 directory, u32 segment, u32 page_request);
int	create_directory(u32 directory);

void	*bmalloc(size_t size);
int	bfree(void *addr);


#endif

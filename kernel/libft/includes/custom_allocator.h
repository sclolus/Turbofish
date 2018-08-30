/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   custom_allocator.h                                 :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/03/24 01:42:50 by bmickael          #+#    #+#             */
/*   Updated: 2017/03/24 02:37:21 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef CUSTOM_ALLOCATOR_H
# define CUSTOM_ALLOCATOR_H

# include <stdlib.h>

struct				s_custom_memory_fn
{
	void			*(*allocator)(size_t);
	void			(*deallocator)(void *);
};

#endif

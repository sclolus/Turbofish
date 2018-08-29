/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_next_line.h                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/11 01:44:48 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/11 01:45:48 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef GNL_H
# define GNL_H

# include "custom_allocator.h"

/*
** Return the next newline of a file descriptor.
*/

int					get_next_line(
		const int fd,
		char **line,
		struct s_custom_memory_fn *mem);

#endif

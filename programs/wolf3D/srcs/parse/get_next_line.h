/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_next_line.h                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <stoupin@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/11 01:44:48 by bmickael          #+#    #+#             */
/*   Updated: 2018/02/01 17:38:42 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef GET_NEXT_LINE_H
# define GET_NEXT_LINE_H

# include "libft.h"
# include <stdlib.h>
# include <unistd.h>

# define BUFF_SIZE 			2048
# define MAX_DESCRIPTORS   	65536

typedef struct		s_buffer
{
	int				fd;
	int				buff_size;
	char			buffer[BUFF_SIZE + 1];
}					t_buffer;

int					get_next_line (const int fd, char **line, size_t max_size);

#endif

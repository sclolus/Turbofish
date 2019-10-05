/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_count_lines.c                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/01/05 19:18:52 by vcombey           #+#    #+#             */
/*   Updated: 2017/04/28 14:44:53 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "libft.h"
#include <errno.h>
#include <unistd.h>
#include <fcntl.h>

int			ft_count_lines(char *name)
{
	int		i;
	int		fd;
	char	*line;

	i = 0;
	if ((fd = open(name, O_RDONLY)) == -1)
		ft_exit(strerror(errno), 2);
	while (get_next_line(fd, &line) > 0)
	{
		i++;
		free(line);
		line = NULL;
	}
	close(fd);
	return (i);
}

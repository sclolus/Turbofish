/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_next_line.c                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <stoupin@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/11 01:37:54 by bmickael          #+#    #+#             */
/*   Updated: 2018/02/01 17:42:40 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "parse/get_next_line.h"

#include <strings.h>
#include <string.h>
#include <stdbool.h>

static char		*s_concat(char **str, const char *buff, size_t *l, size_t n)
{
	char *output;

	if (!(output = (char *)malloc((*l + n + 1) * sizeof(char))))
		return (NULL);
	output[*l + n] = '\0';
	memcpy(output, *str, *l);
	if (*l)
		free(*str);
	*str = output;
	memcpy(*str + *l, buff, n);
	*l += n;
	return (output);
}

static void		init_line_l_size(char **line, size_t *l_size)
{
	*line = NULL;
	*l_size = 0;
}

static void		finalize(t_buffer *index, char *jump_location, size_t i)
{
	memmove(index->buffer, jump_location + 1, BUFF_SIZE - (i + 1));
	index->buffer[(index->buff_size -= i + 1)] = '\0';
}

static int		s_exec(t_buffer *index, char **line, size_t max_len)
{
	char		*jump_location;
	size_t		l_size;
	size_t		i;

	init_line_l_size(line, &l_size);
	while (true)
	{
		if (l_size >= max_len && max_len > 0)
			return (-2);
		if ((index->buff_size < 1) &&
		(index->buff_size = read(index->fd, index->buffer, BUFF_SIZE)) <= 0)
			return ((index->buff_size == 0 && *line) ? 1 : index->buff_size);
		index->buffer[index->buff_size] = '\0';
		if ((jump_location = strchr(index->buffer, '\n')))
			break ;
		if (!s_concat(line, index->buffer, &l_size, index->buff_size))
			return (-1);
		*index->buffer = '\0';
		index->buff_size = 0;
	}
	if (!s_concat(line, index->buffer, &l_size,
		(i = jump_location - index->buffer)))
		return (-1);
	finalize(index, jump_location, i);
	return (1);
}

int				get_next_line(const int fd, char **line, size_t max_len)
{
	static t_buffer	*index[MAX_DESCRIPTORS];
	int				i;

	if (fd < 0 || fd == 1 || fd == 2 || !line)
		return (-1);
	i = 0;
	while (index[i] != NULL && index[i]->fd != fd && i < MAX_DESCRIPTORS)
		i++;
	if (i == MAX_DESCRIPTORS)
		return (-1);
	if (index[i] == NULL)
	{
		if (!(index[i] = (t_buffer *)malloc(sizeof(t_buffer))))
			return (-1);
		bzero(index[i]->buffer, BUFF_SIZE + 1);
		index[i]->buff_size = 0;
		index[i]->fd = fd;
	}
	return (s_exec(index[i], line, max_len));
}

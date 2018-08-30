/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   get_next_line.c                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/11 01:37:54 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/11 02:19:51 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "private_gnl.h"
#include "custom_allocator.h"
#include "libft.h"

static char		*s_concat(
		char **str,
		t_buffer *index,
		size_t n,
		struct s_custom_memory_fn *mem)
{
	char *output;

	if (!(output = (char *)
			mem->allocator((index->l_size + n + 1) * sizeof(char))))
		return (NULL);
	output[index->l_size + n] = '\0';
	ft_memcpy(output, *str, index->l_size);
	if (index->l_size)
		mem->deallocator(*str);
	*str = output;
	ft_memcpy(*str + index->l_size, index->buffer, n);
	index->l_size += n;
	return (output);
}

static int		s_exec(
		t_buffer *index,
		char **line,
		struct s_custom_memory_fn *mem)
{
	char		*jump_location;
	size_t		i;

	*line = NULL;
	index->l_size = 0;
	while (true)
	{
		if ((index->buff_size < 1) &&
		(index->buff_size = read(index->fd, index->buffer, BUFF_SIZE)) <= 0)
			return ((index->buff_size == 0 && *line) ? 1 : index->buff_size);
		index->buffer[index->buff_size] = '\0';
		if ((jump_location = ft_strchr(index->buffer, '\n')))
			break ;
		if (!s_concat(line, index, index->buff_size, mem))
			return (-1);
		*index->buffer = '\0';
		index->buff_size = 0;
	}
	if (!s_concat(line, index, (i = jump_location - index->buffer), mem))
		return (-1);
	ft_memmove(index->buffer, jump_location + 1, BUFF_SIZE - (i + 1));
	index->buffer[(index->buff_size -= i + 1)] = '\0';
	return (1);
}

int				get_next_line(
		const int fd,
		char **line,
		struct s_custom_memory_fn *mem)
{
	static t_buffer	*index[MAX_DESCRIPTORS];
	int				i;

	if (fd < 0 || fd == 1 || fd == 2 || !line || !mem ||
			!mem->allocator || !mem->deallocator)
		return (-1);
	i = 0;
	while (index[i] != NULL && index[i]->fd != fd && i < MAX_DESCRIPTORS)
		i++;
	if (i == MAX_DESCRIPTORS)
		return (-1);
	if (index[i] == NULL)
	{
		if (!(index[i] = (t_buffer *)mem->allocator(sizeof(t_buffer))))
			return (-1);
		ft_bzero(index[i]->buffer, BUFF_SIZE + 1);
		index[i]->buff_size = 0;
		index[i]->fd = fd;
	}
	return (s_exec(index[i], line, mem));
}

/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   load_map.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <stoupin@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 04:33:21 by bmickael          #+#    #+#             */
/*   Updated: 2018/02/01 17:33:36 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include <errno.h>
#include <fcntl.h>
#include "core/wolf3d.h"
#include "parse/get_next_line.h"
#include "parse/internal_parse.h"

static int				check_part(char *s)
{
	if (!(s[0]) || !(s[1]))
		return (-1);
	if (!(s[0] == '_' || (s[0] >= '1' && s[0] <= '0' + N_FLOOR_BMP - 1) |
		(s[0] >= 'a' && s[0] <= 'a' + N_WALL_BMP - 1)))
		return (-1);
	if (!(s[1] == '_' || (s[1] >= '0' && s[1] <= '0' + N_SPRITE_BMP - 1)
			|| s[1] == '%'))
		return (-1);
	if (s[2] != ' ' && s[2] != '\0')
		return (-1);
	return (0);
}

static int				check_all_map(t_map_content *content)
{
	t_list	*lst;
	char	*line;
	int		trigger;
	int		width;

	trigger = FALSE;
	lst = content->data;
	while (lst)
	{
		line = lst->content;
		width = 0;
		while (*line)
		{
			if (check_part(line) == -1)
				return (ft_eprintf("{green}parse:{eoc} wrong data on map\n"));
			line += (line[2] != '\0') ? 3 : 2;
			width++;
		}
		content->width = (!trigger) ? width : content->width;
		if (trigger && content->width != width)
			return (ft_eprintf("{green}parse:{eoc} diff width on lines\n"));
		trigger = TRUE;
		lst = lst->next;
	}
	return (0);
}

static void				map_too_big_error(void)
{
	ft_putstr_fd("Error: map too big\n", 2);
	exit(EXIT_FAILURE);
}

static void				add_line(t_env *e, char *line, size_t *total_len,
									int ret)
{
	*total_len += ft_strlen(line);
	if (*total_len >= MAX_MAP_SIZE)
		map_too_big_error();
	e->content->height++;
	if (!(ft_lst_push_front(&(e->content->data), line, ret + 1)))
		exit(EXIT_FAILURE);
}

int						load_map(t_env *e, char *filename)
{
	char			*line;
	int				fd;
	int				ret;
	size_t			total_len;

	alloc_map_content(&(e->content));
	if ((fd = open(filename, O_RDONLY)) < 0)
	{
		ft_eprintf("Could not open %s : %s\n", filename, strerror(errno));
		return (-1);
	}
	total_len = 0;
	while ((ret = get_next_line(fd, &line, MAX_MAP_SIZE)) > 0)
		add_line(e, line, &total_len, ret);
	if (ret == -2)
		map_too_big_error();
	if (ret == -1 || e->content->height == 0)
		return (-1);
	ft_lst_invert_it(&(e->content->data));
	if (check_all_map(e->content) != 0)
		return (-1);
	close(fd);
	return (0);
}

/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   parsing.c                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vcombey <vcombey@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2016/12/12 16:25:45 by vcombey           #+#    #+#             */
/*   Updated: 2017/05/02 13:30:37 by vcombey          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stddef.h>
#include <stdlib.h>
#include <errno.h>
#include <unistd.h>
#include <fcntl.h>
#include "../libft/libft.h"
#include "mlx.h"
#include "wolf.h"

static int		ft_fill_init(char **l, int i)
{
	int		n;

	n = ft_strstrlen(l);
	if (i == 0)
		env()->map_width = n;
	else if (n != env()->map_width)
		ft_exit("invalid file", 2);
	if (!(env()->map[i] = malloc(sizeof(int) * (n))))
		ft_exit("malloc error", 2);
	return (n);
}

static void		ft_fill_line(char **l, int i)
{
	int				j;
	int				n;
	static int		nb_spawn = 0;
	int				nbr;

	n = ft_fill_init(l, i);
	j = 0;
	while (j < n)
	{
		if (!(ft_atoi_safe(l[j], &nbr) || !(nbr == 5 || nbr == 0 || nbr == -1
						|| nbr == 1)))
			ft_exit("error : bad characters in the file", 2);
		if (nbr == 2 && nb_spawn != 0)
			ft_exit("error : file must have exactly one spawn", 2);
		if (nbr == 2)
		{
			nb_spawn++;
			add_start_position(i, j);
		}
		else
			env()->map[i][j] = nbr;
		j++;
	}
	if ((i == env()->map_height - 1) && nb_spawn != 1)
		ft_exit("error : file must have exactly one spawn", 2);
}

static void		ft_fill_tab(int fd)
{
	char	*line;
	int		i;
	char	**t;

	i = 0;
	while (get_next_line(fd, &line))
	{
		if (!(t = ft_strsplit(line, ' ')))
			ft_exit("malloc error", 2);
		ft_fill_line(t, i);
		i++;
		tab_free(t);
		free(line);
		line = NULL;
	}
}

static void		check_surround(void)
{
	int		i;
	int		**map;
	int		map_height;
	int		map_width;

	i = 0;
	map = env()->map;
	map_width = env()->map_width;
	map_height = env()->map_height;
	while (i < map_width)
	{
		if (map[0][i] != 1 || (env()->map[map_height - 1][i] != 1))
			ft_exit("error file must be surouned by 1", 2);
		i++;
	}
	i = 1;
	while (i < map_height - 1)
	{
		if (map[i][0] != 1 || map[i][map_width - 1] != 1)
			ft_exit("error file must be surouned by 1", 2);
		i++;
	}
}

void			ft_parse_input(char *name)
{
	int		fd;
	int		nb_lines;

	nb_lines = ft_count_lines(name);
	if (nb_lines == 0)
		ft_exit("invalid file", 2);
	env()->map_height = nb_lines;
	if (!(env()->map = (int **)ft_memalloc((nb_lines + 1) * sizeof(int *))))
		ft_exit("malloc error", 2);
	fd = open(name, O_RDONLY);
	env()->map[nb_lines] = NULL;
	ft_fill_tab(fd);
	close(fd);
	check_surround();
}

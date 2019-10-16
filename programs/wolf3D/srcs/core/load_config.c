/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   load_config.c                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <bmickael@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/05 23:42:57 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/06 00:48:43 by erucquoy         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "core/wolf3d.h"
#include "bmp/bmp.h"

t_bmp *load_bitmap(char **name, int n)
{
	t_bmp *array;
	t_bmp *bmp;
	int i;

	if (!(array = malloc(sizeof(t_bmp) * n)))
		return (NULL);
	i = 0;
	while (i < n) {
		bmp = array + i;
		if (!(bmp_load(name[i], &(bmp->dim.x), &(bmp->dim.y),
			       (int**)&(bmp->pix)))) {
			dprintf(STDERR_FILENO, "Critical error when BMP loading\n");
			return (NULL);
		}
		i++;
	}
	return (array);
}

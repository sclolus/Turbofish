/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   weapon.c                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <bmickael@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/06 07:18:25 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/06 09:41:36 by erucquoy         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdio.h>
#include "wolf3d.h"
#include "bmp.h"

/*
** README !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
** L'interpolation rends l'image plutot degeux. On va donc faire des images
** "gun/weapon" de taille parfaites (genre du 100*100 300*300 on verra)
** Et on va load direct l'image sans interpol #appelle la font_nb
*/

void		load_weapon(t_env *e)
{
	static int	loaded = 0;
	t_bmp		*temp;

	if (loaded == 0)
	{
		ft_printf("{yellow}[ LOADING ]{eoc} Weapon\n");
		if (!(temp = load_bitmap((char*[]){"images/gun0.bmp"}, 1)))
			exit(EXIT_FAILURE);
		if (!(e->weapon.data = malloc(sizeof(t_bmp))))
			exit(EXIT_FAILURE);
		temp = &temp[0];
		e->weapon.data->dim.x = 100;
		e->weapon.data->dim.y = 100;
		if (!(e->weapon.data->pix = ft_memalloc(e->weapon.data->dim.x
										* e->weapon.data->dim.y * sizeof(int))))
			exit(EXIT_FAILURE);
		copy_img(e->weapon.data, temp);
		free(temp->pix);
		free(temp);
		loaded = 1;
		ft_printf("{green}[ LOADED ]{eoc} Weapon\n");
	}
}

/*
** //((SCREENSIZE) - (HEIGHT / 2)) - (e->weapon.data->dim.y / 2) -
** (e->weapon.data->dim.y * WIDTH));
*/

void		draw_weapon(t_env *e)
{
	int i;
	int j;

	load_weapon(e);
	i = 0;
	j = 0;
	while (j < (e->weapon.data->dim.y * e->weapon.data->dim.x))
	{
		e->img_string[i] = e->weapon.data->pix[j];
		i++;
		j++;
		if (j % 100 == 0)
		{
			i += (WIDTH - 100);
		}
	}
}

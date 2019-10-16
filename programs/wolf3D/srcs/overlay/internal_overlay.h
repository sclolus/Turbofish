/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   internal_overlay.h                                 :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 09:04:16 by bmickael          #+#    #+#             */
/*   Updated: 2017/07/05 09:04:18 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef INTERNAL_OVERLAY_H
# define INTERNAL_OVERLAY_H

typedef struct	s_line
{
	t_coord_i	p1;
	t_coord_i	p2;
	t_coord_i	d;
	t_pix		b_pix;
	t_pix		f_pix;
}				t_line;

void			draw_line(t_pix *scene, t_line *p);

#endif

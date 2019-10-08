/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   overlay.h                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 09:04:09 by bmickael          #+#    #+#             */
/*   Updated: 2017/07/05 09:04:10 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef OVERLAY_H
# define OVERLAY_H

# include "graphic_types.h"

void			draw_box(t_coord_i p1, t_coord_i p2, t_pix pix, t_pix *scene);

void			fill_box(t_coord_i p1, t_coord_i p2, t_pix pix, t_pix *scene);

void			draw_circle(t_pix *scene, t_coord_i position, int radius,
																t_pix color);

void			draw_arrow(t_pix *scene, t_coord_i c, float angle);

#endif

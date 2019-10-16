/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   bmp.h                                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/05/18 21:54:53 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/03 15:58:23 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef BMP_H
# define BMP_H

int			bmp_load(char *filename, int *width, int *height, int **data);

int			bmp_save(char *filename, int width, int height, int *data);

#endif

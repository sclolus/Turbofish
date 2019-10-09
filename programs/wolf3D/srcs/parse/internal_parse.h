/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   internal_parse.h                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 04:33:12 by bmickael          #+#    #+#             */
/*   Updated: 2017/07/05 04:33:13 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef INTERNAL_PARSE_H
# define INTERNAL_PARSE_H

#include "common.h"

#include <string.h>

typedef struct s_map_content
{
	t_list *data;
	int width;
	int height;
} t_map_content;

void alloc_map_content(t_map_content **content);

#endif

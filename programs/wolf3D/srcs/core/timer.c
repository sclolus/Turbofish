/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   timer.c                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/10 08:33:11 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/10 08:33:13 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <sys/time.h>
#include <stdio.h>
#include "core/wolf3d.h"

/*
** #include <time.h>
** int clock_gettime( clockid_t clock_id, struct timespec *tp );
** The clock_gettime() function gets the current time of the clock specified by
** clock_id, and puts it into the buffer pointed to by tp. The only supported
** clock ID is CLOCK_REALTIME.
** The tp parameter points to a structure containing at least the following
** members:
** time_t tv_sec	-> The number of seconds since 1970.
** time_t tv_nsec	-> The number of nanoseconds expired in the current second.
** This value increases by some multiple of nanoseconds, based on the system
** clock's resolution.
*/

/*
** unsigned long int	get_time(void)
** {
**	struct timespec		spec;
**
**	clock_gettime(CLOCK_REALTIME, &spec);
**	return ((spec.tv_sec * 1000) + spec.tv_nsec / 1.0e6);
**}
*/

unsigned long int	get_time(void)
{
	struct timeval		spec;

	gettimeofday(&spec, NULL);
	return ((spec.tv_sec * 1000) + spec.tv_usec / 1.0e3);
}

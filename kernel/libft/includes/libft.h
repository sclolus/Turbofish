/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   libft.h                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 20:25:00 by bmickael          #+#    #+#             */
/*   Updated: 2018/04/21 23:27:01 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef LIBFT_H
# define LIBFT_H

#include "i386_type.h"

# define HEX_T(x)	"0123456789ABCDEF"[x]

void				*ft_memset(void *b, int c, size_t len);
void				ft_bzero(void *s, size_t n);

void				*ft_memcpy(void *dst, const void *src, size_t n);
//void				*ft_memcpy(void *restrict dst, const void *restrict src,
//		size_t n);
void				*ft_memccpy(void *restrict dest,
		const void *restrict src, int c, size_t n);
void				*ft_memmove(void *dst, const void *src, size_t len);
void				*ft_memchr(const void *s, int c, size_t n);
int					ft_memcmp(const void *s1, const void *s2, size_t n);

size_t				ft_strlen(const char *s);
char				*ft_strcpy(char *dst, const char *src);
char				*ft_strncpy(char *dst, const char *src, size_t len);
char				*ft_strcat(char *restrict s1, const char *restrict s2);
char				*ft_strncat(char *restrict s1, const char *restrict s2,
		size_t n);
size_t				ft_strlcat(char *restrict dst, const char *restrict src,
		size_t size);
char				*ft_strchr(const char *s, int c);
char				*ft_strrchr(const char *s, int c);
char				*ft_strstr(const char *big, const char *little);
char				*ft_strnstr(const char *big, const char *little,
		size_t len);
int					ft_strcmp(const char *s1, const char *s2);
int					ft_strncmp(const char *s1, const char *s2, size_t n);
int					ft_atoi(const char *str);

int					ft_isalpha(int c);
int					ft_isdigit(int c);
int					ft_isalnum(int c);
int					ft_isascii(int c);
int					ft_isprint(int c);
int					ft_toupper(int c);
int					ft_tolower(int c);

void				ft_strclr(char *s);
void				ft_striter(char *s, void (*f)(char *));
void				ft_striteri(char *s, void (*f)(unsigned int, char *));
int					ft_strequ(char const *s1, char const *s2);
int					ft_strnequ(char const *s1, char const *s2, size_t n);

void				ft_putchar(char c);
void				ft_putstr(char const *s);
void				ft_putendl(char const *s);
void				ft_putnbr(int n);
void				ft_putchar_fd(char c, int fd);
void				ft_putstr_fd(char const *s, int fd);
void				ft_putendl_fd(char const *s, int fd);
void				ft_putnbr_fd(int n, int fd);

int					ft_secure_atoi(const char *nptr, bool *error);

void				*ft_aligned_memcpy(void *restrict dst,
		const void *restrict src, size_t n);
void				ft_aligned_bzero(void *s, size_t n);

int					ft_printf(const char *format, ...);
int					ft_eprintf(const char *format, ...);
int					ft_fprintf(int const fd, const char *format,
																		...);
int					ft_dprintf(bool display, const char *format, ...);
int					ft_sprintf(char *str, const char *format, ...);

#endif

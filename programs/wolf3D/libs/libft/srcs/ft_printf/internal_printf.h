/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   internal_printf.h                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 19:26:04 by bmickael          #+#    #+#             */
/*   Updated: 2017/04/11 00:07:08 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef INTERNAL_PRINTF_H
# define INTERNAL_PRINTF_H

# include <stdarg.h>
# include <stdlib.h>
# include <inttypes.h>
# include <wchar.h>
# include <unistd.h>

# define FALSE					0
# define TRUE					1

typedef unsigned char			t_u_char;
typedef unsigned short int		t_su_int;
typedef unsigned int			t_u_int;
typedef unsigned long int		t_lu_int;
typedef unsigned long long int	t_llu_int;
typedef short int				t_s_int;
typedef long int				t_l_int;
typedef long long int			t_ll_int;

# define MASK7                                  0x0000007f
# define MASK11                                 0x000007ff
# define MASK16                                 0x0000ffff
# define MASK21                                 0x001fffff
# define MASK26                                 0x03ffffff
# define MASK31                                 0x7fffffff

# define HEXTABLE_MAJ(x) 		"0123456789ABCDEF"[x]
# define HEXTABLE_MIN(x) 		"0123456789abcdef"[x]
# define STDOUT 				1
# define STDERR 				2
# define MODIFIER_QUANTITY 		8
# define FLAGS_QUANTITY 		5
# define LENGTH_TYPE_QUANTITY	4
# define SPECIFIERS_QUANTITY 	16
# define UTF8_MAX_SIZE			4

typedef struct					s_status
{
	va_list						ap;
	const char *restrict		s;
	char						*ptr;
	int							size;
}								t_status;

typedef enum					e_flags
{
	PLUS = 0x01,
	SPACE = 0x02,
	HASH = 0x04,
	MINUS = 0x08,
	ZERO = 0x10
}								t_flags;

typedef struct					s_s_flags
{
	char						flag;
	t_flags						value;
}								t_s_flags;

typedef enum					e_length
{
	VOID = 0x00,
	H = 0x01,
	L = 0x02,
	LEVEL1 = 0x03,
	Z = 0x04,
	J = 0x08,
	MAJOR = 0x80,
	HH = 0x81,
	LL = 0x82
}								t_length;

typedef struct					s_s_length {
	char						sequence;
	t_length					value;
}								t_s_length;

typedef struct s_args			t_args;
struct							s_args
{
	t_flags						b;
	t_length					l;
	int							w;
	int							p;
	char						s;
	void						(*f)(t_args *, t_status *op);
};

typedef struct					s_specifier {
	char						specifier;
	unsigned char				sp_len;
	void						(*f)(t_args *, t_status *op);
}								t_specifier;

int								assign_segment (int w_size, t_status *op);
void							new_chain (t_status *op);
void							assign_modifier (t_status *op);
void							get_args (const char *restrict s, int *i,
												t_args *args, t_status *op);
void							s_logical_xmin (t_args *args, t_status *op);
void							s_logical_xmaj (t_args *args, t_status *op);
void							s_logical_o (t_args *args, t_status *op);
void							s_logical_b (t_args *args, t_status *op);
void							s_pointer (t_args *args, t_status *op);
void							s_numeric_u (t_args *args, t_status *op);
void							s_numeric_i (t_args *args, t_status *op);
void							s_string (t_args *args, t_status *op);
void							s_char (t_args *args, t_status *op);

int								get_size_for_string(wchar_t c);

void							*ft_memset (void *b, int c, size_t len);
void							*ft_memcpy (void *dest, const void *src,
																size_t n);
size_t							ft_strlen (const char *s);
void							cast_u (uintmax_t *n, t_length mask);
void							cast_i (intmax_t *n, t_length mask);
#endif

/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   internal_printf.h                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/04/10 19:26:04 by bmickael          #+#    #+#             */
/*   Updated: 2018/04/28 20:30:16 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef INTERNAL_PRINTF_H
# define INTERNAL_PRINTF_H

# include <stdarg.h>

# define MAX_BUF_LEN			4096

typedef unsigned char			t_u_char;
typedef unsigned short int		t_su_int;
typedef unsigned int			t_u_int;
typedef unsigned long int		t_lu_int;
typedef unsigned long long int	t_llu_int;
typedef short int				t_s_int;
typedef long int				t_l_int;
typedef long long int			t_ll_int;
typedef unsigned int			size_t;
typedef int						bool;
typedef unsigned int			uintmax_t;
typedef signed int				intmax_t;

# define NULL 0
# define false 0
# define true 1

# define HEXTABLE_MAJ(x) 		"0123456789ABCDEF"[x]
# define HEXTABLE_MIN(x) 		"0123456789abcdef"[x]
# define STDOUT 				1
# define STDERR 				2
# define MODIFIER_QUANTITY 		8
# define FLAGS_QUANTITY 		5
# define LENGTH_TYPE_QUANTITY	4
# define SPECIFIERS_QUANTITY 	16

typedef struct					s_status
{
	va_list						ap;
	const char *restrict		s;
	int							fd;
	int							buff_len;
	int							total_size;
	char						*str;
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
	int							(*f)(t_args *, t_status *op);
};

typedef struct					s_specifier {
	char						specifier;
	unsigned char				sp_len;
	int							(*f)(t_args *, t_status *op);
}								t_specifier;

int								new_chain (t_status *op);
void							assign_modifier (t_status *op);
void							get_args (const char *restrict s, int *i,
												t_args *args, t_status *op);
int								s_logical_xmin (t_args *args, t_status *op);
int								s_logical_xmaj (t_args *args, t_status *op);
int								s_logical_o (t_args *args, t_status *op);
int								s_logical_b (t_args *args, t_status *op);
int								s_pointer (t_args *args, t_status *op);
int								s_numeric_u (t_args *args, t_status *op);
int								s_numeric_i (t_args *args, t_status *op);
int								s_string (t_args *args, t_status *op);
int								s_char (t_args *args, t_status *op);

void							*ft_memset (void *b, int c, size_t len);
void							*ft_memcpy (void *dest, const void *src,
																size_t n);
size_t							ft_strlen (const char *s);
void							cast_u (uintmax_t *n, t_length mask);
void							cast_i (intmax_t *n, t_length mask);

void							string_to_buffer(const char *s, int len,
												t_status *op);
void							char_to_buffer(char c, int len, t_status *op);
void							fflush_buffer(t_status *op);

#endif

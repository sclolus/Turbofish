#ifndef INTERNAL_PRINTF_H
# define INTERNAL_PRINTF_H

#include <stddef.h>
#include <stdlib.h>

typedef __builtin_va_list	va_list;
# define va_start(v,l)		__builtin_va_start(v,l)
# define va_end(v)		__builtin_va_end(v)
# define va_arg(v,l)		__builtin_va_arg(v,l)

# define MAX_BUF_LEN		4096

typedef unsigned char		t_u_char;
typedef unsigned short int	t_su_int;
typedef unsigned int		t_u_int;
typedef unsigned long int	t_lu_int;
typedef unsigned long long int	t_llu_int;
typedef short int		t_s_int;
typedef long int		t_l_int;
typedef long long int		t_ll_int;

typedef unsigned long long uintmax_t;
typedef signed int		intmax_t;
typedef signed int		wchar_t;
typedef signed int		bool;

# define false			0
# define true			1

# define MASK7			0x0000007f
# define MASK11			0x000007ff
# define MASK16			0x0000ffff
# define MASK21			0x001fffff
# define MASK26			0x03ffffff
# define MASK31			0x7fffffff

# define HEXTABLE_MAJ(x) 	"0123456789ABCDEF"[x]
# define HEXTABLE_MIN(x) 	"0123456789abcdef"[x]
# define STDOUT 		1
# define STDERR 		2
# define MODIFIER_QUANTITY 	13
# define FLAGS_QUANTITY 	5
# define LENGTH_TYPE_QUANTITY	4
# define SPECIFIERS_QUANTITY 	16
# define UTF8_MAX_SIZE		4

typedef enum e_params {
	Fd,
	GivenString,
	AllocatedString,
} t_params;

// Usable in printf, vprintf, fprintf, vfprintf, dprintf and vdprintf context
struct fd {
	int fd;
};

// Usable in sprintf, snprintf, vsprintf and vsnprintf context
struct given_string {
	char *str;
	size_t max_size;
};

// Usable in snprintf and vsnprintf context
struct allocated_string {
	char *str;
};

union opt {
	struct fd fd;
	struct given_string given_string;
	struct allocated_string allocated_string;
};

typedef struct			s_status
{
	va_list			ap;
	const char 	        *s;
	int			buff_len;
	int			total_size;

	t_params params;
	union opt opt;
}				t_status;

typedef enum			e_flags
{
	PLUS = 0x01,
	SPACE = 0x02,
	HASH = 0x04,
	MINUS = 0x08,
	ZERO = 0x10
}				t_flags;

typedef struct			s_s_flags
{
	char			flag;
	t_flags			value;
}				t_s_flags;

typedef enum			e_length
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
}				t_length;

typedef struct			s_s_length {
	char			sequence;
	t_length		value;
}				t_s_length;

typedef struct s_args		t_args;
struct				s_args
{
	t_flags			b;
	t_length		l;
	int			w;
	int			p;
	char			s;
	int			(*f)(t_args *, t_status *op);
};

typedef struct			s_specifier {
	char			specifier;
	unsigned char		sp_len;
	int			(*f)(t_args *, t_status *op);
}				t_specifier;

int	new_chain(t_status *op);
void	assign_modifier (t_status *op);
void	get_args(const char *restrict s, int *i, t_args *args, t_status *op);
int	s_logical_xmin(t_args *args, t_status *op);
int	s_logical_xmaj(t_args *args, t_status *op);
int	s_logical_o(t_args *args, t_status *op);
int	s_logical_b(t_args *args, t_status *op);
int	s_pointer(t_args *args, t_status *op);
int	s_numeric_u(t_args *args, t_status *op);
int	s_numeric_i(t_args *args, t_status *op);
int	s_string(t_args *args, t_status *op);
int	s_char(t_args *args, t_status *op);

int	get_size_for_string(wchar_t c);

void	*ft_memset(void *b, int c, size_t len);
void	*ft_memcpy(void *dest, const void *src, size_t n);
size_t	strlen(const char *s);
void	cast_u(uintmax_t *n, t_length mask);
void	cast_i(intmax_t *n, t_length mask);

void	string_to_buffer(const char *s, int len, t_status *op);
void	char_to_buffer(char c, int len, t_status *op);
void	fflush_buffer(t_status *op);

/*
 * norme.c: Not logical, just for the trash norm.
 */

void	s_char_wchar(t_args *args, t_status *op, wchar_t c);

#endif

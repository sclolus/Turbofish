#include "internal_printf.h"

#include <assert.h>
#include <sys/types.h>

void	cast_u(uintmax_t *n, t_length mask, t_status *op)
{
	if (mask == VOID)
		*n = (uintmax_t)va_arg(op->ap, unsigned);
	else if (mask == H)
		*n = (uintmax_t)((t_su_int)va_arg(op->ap, unsigned));
	else if (mask == HH)
		*n = (uintmax_t)((t_u_char)va_arg(op->ap, unsigned));
	else if (mask == L)
		*n = (uintmax_t)((t_lu_int)va_arg(op->ap, long unsigned));
	else if (mask == LL)
		*n = (uintmax_t)((t_llu_int)va_arg(op->ap, long long unsigned));
	else if (mask == Z)
		*n = (uintmax_t)va_arg(op->ap, size_t);
	else if (mask == J)
		*n = va_arg(op->ap, uintmax_t);
	else
		// This never should happen
		assert(false);
}

void	cast_i(intmax_t *n, t_length mask, t_status *op)
{
	if (mask == VOID)
		*n = (intmax_t)va_arg(op->ap, signed);
	else if (mask == H)
		*n = (intmax_t)((t_s_int)va_arg(op->ap, signed));
	else if (mask == HH)
		*n = (intmax_t)((char)va_arg(op->ap, signed));
	else if (mask == L)
		*n = (intmax_t)((t_l_int)va_arg(op->ap, long signed));
	else if (mask == LL)
		*n = (intmax_t)((t_ll_int)va_arg(op->ap, long long signed));
	else if (mask == Z)
		*n = (intmax_t)va_arg(op->ap, ssize_t);
	else if (mask == J)
		*n = va_arg(op->ap, intmax_t);
	else
		// This never should happen
		assert(false);
}

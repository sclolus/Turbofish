#include <ltrace.h>
#include <float.h>
#include <math.h>

/// These functions shall compute the value of x raised to the power y, xy. If x is negative, the application shall ensure that y is an integer value.

#warning some posix constraint tests are failling.

double pow(double x, double y)
{
	double	res;

	__turbofish_pow(x, y, &res);
	return res;
}

// Integration of unit tests for pow is difficult due to the difference in
// target architecture between x86-64 and the turbofish's target.
#ifdef UNIT_TESTS
# include <criterion/criterion.h>

const double EPSILON = 0.01;

Test(pow, nan_param_returns_nan) {
	double	ret;

	ret = pow(NAN, 3.0);
	cr_assert(isnan(ret));

	ret = pow(3.0, NAN);
	cr_assert(isnan(ret));
}


Test(pow, x_is_one) {
	double ret;

	ret = pow(1.0, 234);
	cr_assert_float_eq(ret, 1.0, EPSILON);

	ret = pow(+1, 234);
	cr_assert_float_eq(ret, 1.0, EPSILON);

	ret = pow(1.0, NAN);
	cr_assert_float_eq(ret, 1.0, EPSILON);

	ret = pow(1.0, INFINITY);
	cr_assert_float_eq(ret, 1.0, EPSILON);

}

Test(pow, y_is_zero) {
	double ret;

	ret = pow(234.0, 0.0);
	cr_assert_float_eq(ret, 1.0, EPSILON);

	ret = pow(-42.0, 0.0);
	cr_assert_float_eq(ret, 1.0, EPSILON);

	ret = pow(NAN, 0.0);
	cr_assert_float_eq(ret, 1.0, EPSILON);

	ret = pow(INFINITY, 0.0);
	cr_assert_float_eq(ret, 1.0, EPSILON);
}

Test(pow, y_is_odd_integer_x_is_zero) {
	double ret;

	ret = pow(+0.0, 1.0);
	cr_assert_float_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 1.0);
	cr_assert_float_eq(ret, -0.0, EPSILON);


	ret = pow(+0.0, 3.0);
	cr_assert_float_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 3.0);
	cr_assert_float_eq(ret, -0.0, EPSILON);


	ret = pow(+0.0, 5.0);
	cr_assert_float_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 5.0);
	cr_assert_float_eq(ret, -0.0, EPSILON);
}

Test(pow, y_is_not_odd_x_is_zero) {
	double ret;

	ret = pow(+0.0, 2.0);
	cr_assert_float_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 2.0);
	cr_assert_float_eq(ret, +0.0, EPSILON);


	ret = pow(+0.0, 2.42);
	cr_assert_float_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 2.42);
	cr_assert_float_eq(ret, +0.0, EPSILON);


	ret = pow(+0.0, 3.42);
	cr_assert_float_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 8435.0);
	cr_assert_float_eq(ret, +0.0, EPSILON);
}

Test(pow, x_is_minus_one_y_is_infinity) {
	double ret;

	ret = pow(-1.0, -INFINITY);
	cr_assert_float_eq(ret, 1.0, EPSILON);

	ret = pow(-1.0, +INFINITY);
	cr_assert_float_eq(ret, 1.0, EPSILON);
}

Test(pow, x_magnitude_is_below_1_y_is_minus_INFINITYinity) {
	double ret;

	ret = pow(0.234, -INFINITY);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);

	ret = pow(-0.42, -INFINITY);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);
}

Test(pow, x_magnitude_is_above_1_y_is_minus_INFINITYinity) {
	double ret;

	ret = pow(10.234, -INFINITY);
	cr_assert_float_eq(ret, +0, EPSILON);

	ret = pow(-10.42, -INFINITY);
	cr_assert_float_eq(ret, +0, EPSILON);
}

Test(pow, x_magnitude_is_below_1_y_is_plus_INFINITYinity) {
	double ret;

	ret = pow(0.234, +INFINITY);
	cr_assert_float_eq(ret, +0, EPSILON);

	ret = pow(-0.42, +INFINITY);
	cr_assert_float_eq(ret, +0, EPSILON);
}

Test(pow, x_magnitude_is_above_1_y_is_plus_INFINITYinity) {
	double ret;

	ret = pow(2340.234, +INFINITY);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);

	ret = pow(-330.42, +INFINITY);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);
}

Test(pow, y_is_negative_odd_integer_x_is_minus_INFINITYinity) {
	double ret;

	ret = pow(-INFINITY, -1.0);
	cr_assert_float_eq(ret, -0, EPSILON);

	ret = pow(-INFINITY, -3.0);
	cr_assert_float_eq(ret, -0, EPSILON);
}

Test(pow, y_is_negative_not_odd_integer_x_is_minus_INFINITYinity) {
	double ret;

	ret = pow(-INFINITY, -2.0);
	cr_assert_float_eq(ret, +0, EPSILON);

	ret = pow(-INFINITY, -4.23434);
	cr_assert_float_eq(ret, +0, EPSILON);
}

Test(pow, y_is_odd_integer_x_is_minus_INFINITYinity) {
	double ret;

	ret = pow(-INFINITY, 1.0);
	cr_assert_float_eq(ret, -INFINITY, EPSILON);


	ret = pow(-INFINITY, 3.0);
	cr_assert_float_eq(ret, -INFINITY, EPSILON);

	ret = pow(-INFINITY, 5.0);
	cr_assert_float_eq(ret, -INFINITY, EPSILON);
}

Test(pow, y_is_not_odd_integer_x_is_minus_INFINITYinity) {
	double ret;

	ret = pow(-INFINITY, 2.320);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);

	ret = pow(-INFINITY, 3.234);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);


	ret = pow(-INFINITY, 2.0);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);

	ret = pow(-INFINITY, 42.0);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);


	ret = pow(-INFINITY, 52.1);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);

	ret = pow(-INFINITY, 43.1);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);
}

Test(pow, y_is_below_0_x_is_plus_INFINITYinity) {
	double ret;

	ret = pow(+INFINITY, -2.320);
	cr_assert_float_eq(ret, +0, EPSILON);

	ret = pow(+INFINITY, -3.234);
	cr_assert_float_eq(ret, +0, EPSILON);


	ret = pow(+INFINITY, -2.0);
	cr_assert_float_eq(ret, +0, EPSILON);

	ret = pow(+INFINITY, -42.0);
	cr_assert_float_eq(ret, +0, EPSILON);


	ret = pow(+INFINITY, -52.1);
	cr_assert_float_eq(ret, +0, EPSILON);

	ret = pow(+INFINITY, -43.1);
	cr_assert_float_eq(ret, +0, EPSILON);
}

Test(pow, y_is_above_0_x_is_plus_INFINITYinity) {
	double ret;

	ret = pow(+INFINITY, 2.320);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);

	ret = pow(+INFINITY, 3.234);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);


	ret = pow(+INFINITY, 2.0);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);

	ret = pow(+INFINITY, 42.0);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);


	ret = pow(+INFINITY, 52.1);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);

	ret = pow(+INFINITY, 43.1);
	cr_assert_float_eq(ret, +INFINITY, EPSILON);
}

Test(pow, basic_powers_of_two) {
	double ret;

	ret = pow(2.0, 2.0);
	cr_assert_float_eq(ret, 4.0, EPSILON);

	ret = pow(2.0, 3.0);
	cr_assert_float_eq(ret, 8.0, EPSILON);


	ret = pow(2.0, 4.0);
	cr_assert_float_eq(ret, 16.0, EPSILON);

	ret = pow(2.0, 5.0);
	cr_assert_float_eq(ret, 32.0, EPSILON);


	ret = pow(2.0, 6.0);
	cr_assert_float_eq(ret, 64.0, EPSILON);

	ret = pow(2.0, 7.0);
	cr_assert_float_eq(ret, 128.0, EPSILON);
}

/* Test(pow, basic) { */
/* 	double ret; */

/* 	ret = pow(2.0, 2.0); */
/* 	cr_assert_float_eq(ret, 4.0, EPSILON); */

/* 	ret = pow(2.0, 3.0); */
/* 	cr_assert_float_eq(ret, 8.0, EPSILON); */


/* 	ret = pow(2.0, 4.0); */
/* 	cr_assert_float_eq(ret, 16.0, EPSILON); */

/* 	ret = pow(2.0, 5.0); */
/* 	cr_assert_float_eq(ret, 32.0, EPSILON); */


/* 	ret = pow(2.0, 6.0); */
/* 	cr_assert_float_eq(ret, 64.0, EPSILON); */

/* 	ret = pow(2.0, 7.0); */
/* 	cr_assert_float_eq(ret, 128.0, EPSILON); */
/* } */
#endif /* UNIT_TESTS */

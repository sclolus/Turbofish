#include <math.h>
#include <float.h>
#include <assert.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdbool.h>

static inline bool  double_eq_epsilon(const double a,
				      const double b,
				      const double epsilon)
{
	return a >= b - epsilon && a <= b + epsilon;
}

#define assert_double_eq(actual, expected, epsilon) do {		\
		assert(double_eq_epsilon(actual, expected, epsilon));	\
	} while (0);

int main(void)
{
	const double EPSILON = 0.1;
	double	     ret;

	ret = pow(NAN, 3.0);
	assert(isnan(ret));

	ret = pow(3.0, NAN);
	assert(isnan(ret));



	ret = pow(1.0, 234);
	assert_double_eq(ret, 1.0, EPSILON);

	ret = pow(+1, 234);
	assert_double_eq(ret, 1.0, EPSILON);

	ret = pow(1.0, NAN);
	assert_double_eq(ret, 1.0, EPSILON);

	ret = pow(1.0, INFINITY);
	assert_double_eq(ret, 1.0, EPSILON);



	ret = pow(234.0, 0.0);
	assert_double_eq(ret, 1.0, EPSILON);

	ret = pow(-42.0, 0.0);
	assert_double_eq(ret, 1.0, EPSILON);

	ret = pow(NAN, 0.0);
	assert_double_eq(ret, 1.0, EPSILON);

	ret = pow(INFINITY, 0.0);
	assert_double_eq(ret, 1.0, EPSILON);


	ret = pow(+0.0, 1.0);
	assert_double_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 1.0);
	assert_double_eq(ret, -0.0, EPSILON);


	ret = pow(+0.0, 3.0);
	assert_double_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 3.0);
	assert_double_eq(ret, -0.0, EPSILON);


	ret = pow(+0.0, 5.0);
	assert_double_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 5.0);
	assert_double_eq(ret, -0.0, EPSILON);


	ret = pow(+0.0, 2.0);
	assert_double_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 2.0);
	assert_double_eq(ret, +0.0, EPSILON);


	ret = pow(+0.0, 2.42);
	assert_double_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 2.42);
	assert_double_eq(ret, +0.0, EPSILON);


	ret = pow(+0.0, 3.42);
	assert_double_eq(ret, +0.0, EPSILON);

	ret = pow(-0.0, 8435.0);
	assert_double_eq(ret, +0.0, EPSILON);


	/* ret = pow(-1.0, -INFINITY); */
	/* assert_double_eq(ret, 1.0, EPSILON); */

	/* ret = pow(-1.0, +INFINITY); */
	/* assert_double_eq(ret, 1.0, EPSILON); */


	ret = pow(0.234, -INFINITY);
	assert_double_eq(ret, +INFINITY, EPSILON);

	/* ret = pow(-0.42, -INFINITY); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */


	ret = pow(10.234, -INFINITY);
	assert_double_eq(ret, +0, EPSILON);

	/* ret = pow(-10.42, -INFINITY); */
	/* assert_double_eq(ret, +0, EPSILON); */


	/* ret = pow(0.234, +INFINITY); */
	/* assert_double_eq(ret, +0, EPSILON); */

	/* ret = pow(-0.42, +INFINITY); */
	/* assert_double_eq(ret, +0, EPSILON); */


	/* ret = pow(2340.234, +INFINITY); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */

	/* ret = pow(-330.42, +INFINITY); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */


	/* ret = pow(-INFINITY, -1.0); */
	/* assert_double_eq(ret, -0, EPSILON); */

	/* ret = pow(-INFINITY, -3.0); */
	/* assert_double_eq(ret, -0, EPSILON); */


	/* ret = pow(-INFINITY, -2.0); */
	/* assert_double_eq(ret, +0, EPSILON); */

	/* ret = pow(-INFINITY, -4.23434); */
	/* assert_double_eq(ret, +0, EPSILON); */


	/* ret = pow(-INFINITY, 1.0); */
	/* assert_double_eq(ret, -INFINITY, EPSILON); */


	/* ret = pow(-INFINITY, 3.0); */
	/* assert_double_eq(ret, -INFINITY, EPSILON); */

	/* ret = pow(-INFINITY, 5.0); */
	/* assert_double_eq(ret, -INFINITY, EPSILON); */


	/* ret = pow(-INFINITY, 2.320); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */

	/* ret = pow(-INFINITY, 3.234); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */


	/* ret = pow(-INFINITY, 2.0); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */

	/* ret = pow(-INFINITY, 42.0); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */


	/* ret = pow(-INFINITY, 52.1); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */

	/* ret = pow(-INFINITY, 43.1); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */


	/* ret = pow(+INFINITY, -2.320); */
	/* assert_double_eq(ret, +0, EPSILON); */

	/* ret = pow(+INFINITY, -3.234); */
	/* assert_double_eq(ret, +0, EPSILON); */


	/* ret = pow(+INFINITY, -2.0); */
	/* assert_double_eq(ret, +0, EPSILON); */

	/* ret = pow(+INFINITY, -42.0); */
	/* assert_double_eq(ret, +0, EPSILON); */


	/* ret = pow(+INFINITY, -52.1); */
	/* assert_double_eq(ret, +0, EPSILON); */

	/* ret = pow(+INFINITY, -43.1); */
	/* assert_double_eq(ret, +0, EPSILON); */


	/* ret = pow(+INFINITY, 2.320); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */

	/* ret = pow(+INFINITY, 3.234); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */


	/* ret = pow(+INFINITY, 2.0); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */

	/* ret = pow(+INFINITY, 42.0); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */


	/* ret = pow(+INFINITY, 52.1); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */

	/* ret = pow(+INFINITY, 43.1); */
	/* assert_double_eq(ret, +INFINITY, EPSILON); */


	ret = pow(2.0, 3);
	/* assert_double_eq(ret, 8.0, EPSILON);  //seems like the above lines fucked up the FPU. this is doomed to fail. */
	assert_double_eq(ret, 8.0, EPSILON);

	ret = pow(2.0, 2);
	assert_double_eq(ret, 4, EPSILON);


	ret = pow(2.0, 4);
	assert_double_eq(ret, 16.0, EPSILON);

	ret = pow(2.0, 5);
	assert_double_eq(ret, 32.0, EPSILON);


	ret = pow(2.0, 6);
	assert_double_eq(ret, 64.0, EPSILON);

	ret = pow(2.0, 7);
	assert_double_eq(ret, 128.0, EPSILON);

	ret = pow(3.0, 2);
	assert_double_eq(ret, 9.0, EPSILON);

	ret = pow(3.0, 3.0);
	assert_double_eq(ret, 27.0, EPSILON);

	ret = pow(42.42, 4.42);
	assert_double_eq(ret, 15626522.2866, EPSILON);

	return EXIT_SUCCESS;
}

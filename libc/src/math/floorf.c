#include <math.h>

float floorf(float x)
{
	// TODO: Considering limits values like inf, nan or 0
	float r = roundf(x);
	if (x > 0) {
		if (r <= x) {
			return r;
		} else {
			return r - 1;
		}
	} else {
		if (r >= x) {
			return r - 1;
		} else {
			return r;
		}
	}
}

/*
 *  (x) (floor) (round)    (Result)  (final)
 *  0.3 => 0      (0) ----> R     ---->  0
 *  0.7 => 0      (1) ----> R - 1 ---->  0
 *  2.2 => 2      (2) ----> R     ---->  2
 *  2.9 => 2      (3) ----> R - 1 ---->  2
 *  3.1 => 3      (3) ----> R     ---->  3
 * -0.2 => -1     (0) ----> R - 1 ----> -1
 * -0.9 => -1    (-1) ----> R     ----> -1
 */

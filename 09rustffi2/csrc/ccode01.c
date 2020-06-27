#include<stdio.h>

typedef void (*SumSquareCB)(int result);

void sum_square_cb01(int a, int b, SumSquareCB cb) {
	int result = a*a + b*b;
	cb(result);
}

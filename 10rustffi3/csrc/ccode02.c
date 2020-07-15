#include <stdio.h>
#include <stdint.h>

extern uint32_t addtwo1(uint32_t, uint32_t);

int main(void) {
  uint32_t sum = addtwo1(10, 20);
  printf("print in c, sum is: %d\n", sum);
}

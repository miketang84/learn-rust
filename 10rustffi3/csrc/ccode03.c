#include <stdio.h>
#include <stdint.h>

extern uint32_t sum_of_array(const uint32_t *numbers, size_t length);

int main(void) {
  uint32_t numbers[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
  unsigned int length = sizeof(numbers) / sizeof(*numbers);
  uint32_t sum = sum_of_array(numbers, length);
  printf("print in c, sum is: %d\n", sum);
}

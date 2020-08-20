#include<stdio.h>
#include<stdlib.h>
#include<malloc.h>


typedef struct Students {
	int num;  		// serial number
	int total; 		// total score
} Student;

extern void fill_students(Student *stu, int);
extern void print_students(Student *stu, int);

void print_students_c(Student *stu, int n) {
	int i;
	for (i=0; i<n; i++) {
		printf("C side print: %d %d\n", stu[i].num, stu[i].total);
	}
}

void main() {
	int len = 10;
	Student students[len];

	// call rust fill and print functions
	fill_students(students, len);
	print_students(students, len);
	
	// call c print function
	print_students_c(students, len);
}

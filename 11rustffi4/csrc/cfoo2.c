#include<stdio.h>
#include<stdlib.h>
#include<malloc.h>

typedef struct Students {
	int num;  		// serial number
	int total; 		// total score
} Student;

extern void fill_students(Student *stu, int);
extern void print_students(Student *stu, int);

Student* create_students(int n) {
	if (n <= 0) return NULL;
	
	Student *stu = NULL;
	stu = (Student*) malloc(sizeof(Student)*n);	

	return stu;
}

void release_students(Student *stu) {
	if (stu != NULL) 
		free(stu);
}

void print_students_c(Student *stu, int n) {
	int i;
	for (i=0; i<n; i++) {
		printf("C side print: %d %d\n", stu[i].num, stu[i].total);
	}
}

void main() {
	int len = 10;
	Student* students = create_students(len);

	// call rust fill and print functions
	fill_students(students, len);
	print_students(students, len);
	
	// call c print function
	print_students_c(students, len);

	release_students(students);
}

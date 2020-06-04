#include<stdio.h>
#include<stdlib.h>
#include<malloc.h>

typedef struct Students {
	int num;  		// serial number
	int total; 		// total score
	char name[20]; 	// name
	float scores[3]; // each scores of 3 subjects
} Student;

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

void print_students(Student *stu, int n) {
	int i;
	for (i=0; i<n; i++) {
		printf("C side print: %d %s %d %.2f %.2f %.2f\n", 
						stu[i].num, 
						stu[i].name, 
						stu[i].total, 
						stu[i].scores[0], 
						stu[i].scores[1], 
						stu[i].scores[2]);
	}
}


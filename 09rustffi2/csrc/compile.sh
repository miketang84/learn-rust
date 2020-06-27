#gcc -o cfoo cfoo.c
gcc -fPIC -shared -o ../libccode01.so ccode01.c
gcc -fPIC -shared -o ../libccode02.so ccode02.c
gcc -fPIC -shared -o ../libccode03.so ccode03.c


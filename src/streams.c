#include <stdio.h>
#include <errno.h>

FILE* stdin_file() {
    return stdin;
}

FILE * stdout_file() {
    return stdout;
}

FILE * stderr_file() {
    return stderr;
}

int errno_val() {
    return errno;
}
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <assert.h>

bool any_substring(char** array, int sz, char* str) {
    for (int i = 0 ; i < sz ; ++i) {
        if (strstr(array[i], str))
            return true;
    }

    return false;
}

int cmp(const void* s1, const void* s2) {
    return strcmp(*(char**)s1, *(char**)s2);
}

// sz1: size of array1, sz2: size of array2, lg: size of the returned array
char** inArray(char* array1[], int sz1, char* array2[], int sz2, int* lg) {
    char** result = malloc(sizeof(char*) * (sz1+1));
    *lg = 0;

    for (int i = 0 ; i < sz1 ; ++i) {
        if (any_substring(array2, sz2, array1[i])) {
            result[*lg] = malloc(strlen(array1[i])+1);
            strcpy(result[*lg], array1[i]);
            (*lg)++;
        }
    }

    qsort(result, *lg, sizeof(result[0]), cmp);
    return result;
}

char* joinStringsStringArray(char *strings[], int count) {
    char* result = malloc(strlen(strings[0]) * count);
    strcpy(result, "{");

    char temp[100];
    for (int i = 0 ; i < count ; ++i) {
        sprintf(temp, "'%s',", strings[i]);
        strcat(result, temp);
    }
    result[strlen(result)-1] = '\0';
    strcat(result, "}");

    return result;
}

void dotest(char** u, int szu, char** v, int szv, char* expr) {
    int lg;
    char** act = inArray(u, szu, v, szv, &lg);
    char* sact = joinStringsStringArray(act, lg);

    if(strcmp(sact, expr) != 0) {
        printf("Error. Expected \n%s\n but got \n%s\n", expr, sact);
    }

    /* assert(strcmp(sact, expr) == 0); */
    /* if (strcmp(sact, "{}") != 0) { */
        /* free(sact); sact = NULL; */
    /* } */
    /* free(act); act = NULL; */

}

int main() {
    {

        char* arr1[3] = { "arp", "live", "strong" };
        char* arr2[5] = { "lively", "alive", "harp", "sharp", "armstrong" };
        dotest(arr1, 3, arr2, 5, "{'arp','live','strong'}");
    }
    {
        char* arr1[7] = { "cod", "code", "wars", "ewar", "pillow", "bed", "phht" };
        char* arr2[8] = { "lively", "alive", "harp", "sharp", "armstrong", "codewars", "cod", "code" };
        dotest(arr1, 7, arr2, 8, "{'cod','code','ewar','wars'}");
    }
}

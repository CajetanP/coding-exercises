#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include "roman_numerals.h"

typedef struct _PairIS {
    int one;
    char* two;
} PairIS;

char* to_roman_numeral(int num) {
    PairIS table[13] = {
        { 1, "I" },
        { 4, "IV" },
        { 5, "V" },
        { 9, "IX" },
        { 10, "X" },
        { 40, "XL" },
        { 50, "L" },
        { 90, "XC" },
        { 100, "C" },
        { 400, "CD" },
        { 500, "D" },
        { 900, "CM" },
        { 1000, "M" },
    };

    char* result = malloc(10);
    result[0] = '\0';

    while (num > 0) {
        int idx = 12;

        while (table[idx].one > num)
            idx--;

        num -= table[idx].one;
        strcat(result, table[idx].two);
    }

    return result;
}

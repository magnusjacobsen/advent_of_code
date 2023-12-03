#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>

#define MAXC 1024
#define NUM_LINES 140

typedef struct {
    int i;
    int j;
    int len;
    int num;
} number;

int get_num(int start, int end, char bufline[]) {
    int out = 0;
    for (int i = start; i <= end; i++) {
        int val = bufline[i] - '0';
        int scalar = (int)pow(10, (end - i));
        out += val * scalar;
    }
    return out;
}

bool is_digit(char c) {
    return c >= '0' && c <= '9';
}

int get_len(int start, char bufline[], int max_i) {
    int len = 0;
    if (is_digit(bufline[start])) {
        len++;
        for (int i = start + 1; i < max_i; i++) {
            if (!is_digit(bufline[i])) {
                return len;
            }
            len++;
        }
    }
    return len;
}

bool is_star(int i, int j, int max_i, int max_j, char buf[][MAXC]) {
    if (i > 0 && j > 0 & i < max_i && j < max_j) {
        char c = buf[i][j];
        return c == '*';
    }
    return false;
}

bool is_relevant(int row_idx, int start, int end, int max_i, int max_j, char buf[][MAXC]) {
    if (is_star(row_idx - 1, start - 1, max_i, max_j, buf))
        return true; // top left
    else if (is_star(row_idx, start - 1, max_i, max_j, buf))
        return true; // left
    else if (is_star(row_idx + 1, start - 1, max_i, max_j, buf))
        return true; // bottomm left
    else if (is_star(row_idx - 1, end + 1, max_i, max_j, buf))
        return true; // top right
    else if (is_star(row_idx, end + 1, max_i, max_j, buf))
        return true; // right
    else if (is_star(row_idx + 1, end + 1, max_i, max_j, buf))
        return true; // bottom right
    for (int i = start; i <= end; i++) {
        if (is_star(row_idx - 1, i, max_i, max_j, buf))
            return true; // above
        else if (is_star(row_idx + 1, i, max_i, max_j, buf))
            return true; // below
    }
    return false;
}

int main (int argc, char **argv) {
    char buf[NUM_LINES][MAXC];
    FILE *fp = stdin;
    int height = 0;
    while (fgets(buf[height], MAXC, fp)) {
        buf[height][strcspn(buf[height], "\n")] = 0;
        height++;
    }
    int width = strlen(buf[0]);
    int num_numbers = 0;
    number numbers[NUM_LINES * NUM_LINES];
    for (int i = 0; i < height; i++) {
        int j = 0;
        while (j < width) {
            int num_len = get_len(j, buf[i], width);
            if (num_len > 0) {
                int end = j + num_len - 1;
                if (is_relevant(i, j, end, height - 1, width - 1, buf)) {
                    int num_value = get_num(j, end, buf[i]);
                    number new_number = {i, j, num_len, num_value};
                    numbers[num_numbers] = new_number;
                    num_numbers++;
                }
                j += num_len;
            } else {
                j++;
            }
        }
    }
    int sum = 0;
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++) {
            if (buf[i][j] == '*') {
                int num_gear_parts = 0;
                int gear_ratio = 1;
                for (int k = 0; k < num_numbers; k++) {
                    number number = numbers[k];
                    int i_diff = number.i - i;
                    if (
                        (number.i + 1 == i || number.i == i || number.i - 1 == i) 
                        && j > number.j - 2
                        && j < number.j + number.len + 1
                    ) {
                        num_gear_parts++;
                        gear_ratio *= number.num;
                    }
                }
                if (num_gear_parts == 2) {
                    sum += gear_ratio;
                }
            }
        }
    }

    printf("%d\n", sum);
    if (fp != stdin) {
        fclose(fp);
    }
    return 0;
}
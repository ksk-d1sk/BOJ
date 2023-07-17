// 윷놀이

#include <stdio.h>

int main() {
	int a, b, c, d;
	char n = 3;
	while (n--) {
		scanf("%d %d %d %d", &a, &b, &c, &d);
		int sum = a + b + c + d;
		printf("%c\n", 68 - (sum / 4 * -5 + sum));
	}

	return 0;
}
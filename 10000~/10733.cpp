// Á¦·Î 

#include <iostream>

int* dat;
int pos = 0;

void push(int x) {
	dat[pos++] = x;
}

void pop() {
	if (pos > 0) pos--;
}

int main() {
	std::ios::sync_with_stdio(0);
	std::cin.tie(0);

	int k;
	std::cin >> k;

	dat = new int[k];

	for (int i = 0; i < k; i++) {
		int n;
		std::cin >> n;
		if (n) push(n);
		else pop();
	}

	long long int sum = 0;
	for (int i = 0; i < pos; i++) {
		sum += dat[i];
	}

	std::cout << sum;

	delete[] dat;

	return 0;
}

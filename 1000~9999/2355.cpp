// 시그마

#include <iostream>

int main() {
	
	std::ios::sync_with_stdio(0);

	long long int a, b;
	std::cin >> a >> b;

	int result = a <= b ? (a + b) * (b - a + 1) / 2 : (a + b) * (a - b + 1) / 2;

	std::cout << result;

	return 0;
}
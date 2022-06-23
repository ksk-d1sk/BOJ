// 빠른 A+B

#include <iostream>

int main(void)
{
	std::ios::sync_with_stdio(false);
	std::cin.tie(nullptr);
	
	int t;
	std::cin >> t;
	
	for (int i = 0; i < t; i++)
	{
		int a, b;
		std::cin >> a >> b;
		std::cout << a + b << '\n';
	}
	
	return 0;
}

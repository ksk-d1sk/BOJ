// 과제 안 내신 분..?

#include <iostream>

int main()
{
	std::ios::sync_with_stdio(false);
    
    short arr[30];
    
    for (int i = 0; i < 30; i++)
		arr[i] = 1;
	
	for (int i = 0; i < 28; i ++)
	{
		int n;
		std::cin >> n;
		arr[n - 1] = 0;
	}
	
	for (int i = 0; i < 30; i++)
		if (arr[i]) std::cout << i + 1 << '\n';
	
	return 0;
}

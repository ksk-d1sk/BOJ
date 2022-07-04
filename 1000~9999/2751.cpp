// 수 정렬하기 2 

#include <iostream>

int main()
{
	std::ios::sync_with_stdio(0);
	std::cin.tie(0);
	
	int n;
	std::cin >> n;
	
	bool arr[2000001] = {false, };
	
	for (int i = 0; i < n; i++)
	{
		int temp;
		std::cin >> temp;
		arr[temp + 1000000] = true;
	}
	
	for (int i = 0; i < 2000001; i++)
		if (arr[i]) std::cout << i - 1000000 << '\n';
	
	return 0;
}

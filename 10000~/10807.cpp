// 개수 세기

#include <iostream>

int main(void)
{
	short n;
	std::cin >> n;
	
	short* arr = new short[n];
	
	for (int i = 0; i < n; i++)
		std::cin >> arr[i];
	
	short v;
	std::cin >> v;
	
	short count = 0;
	for (int i = 0; i < n; i++)
		if (v == arr[i]) count++;
	
	delete[] arr;
	
	std::cout << count;
	
	return 0;
} 

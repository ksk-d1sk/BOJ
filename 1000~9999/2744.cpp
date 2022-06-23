// 대소문자 바꾸기

#include <iostream>

int main()
{
	std::ios::sync_with_stdio(false);
	
	std::string str;
	std::cin >> str;
	
	for (char s : str)
	{
		if (s >= 'A' && s <= 'Z')
		{
			std::cout << (char)(s + 32);
		}
		else
		{
			std::cout << (char)(s - 32);
		}
	}
	
	return 0;
}

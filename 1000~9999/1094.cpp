// 막대기

#include <iostream>

int main()
{
	std::ios::sync_with_stdio(0);
	
	short cnt = 0;

	short x;
	std::cin >> x;

	while (x > 0)
	{
		int i;
		for (i = 1; true; i *= 2)
			if (i > x)
			{
				i /= 2;
				cnt++;
				break;
			}
		x -= i;
	}

	std::cout << cnt;

	return 0;
}
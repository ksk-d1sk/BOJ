// 게임

#include <iostream>

int onePercnet(long long x, long long y)
{
	int z = y * 100 / x;

	if (z >= 99)
		return -1;

	int ans = -1;
	int low = 0;
	int high = 1000000000;

	while (low <= high)
	{
		int mid = (low + high) / 2;
		int val = (y + mid) * 100 / (x + mid);
		if (z >= val)
		{
			low = mid + 1;
			ans = mid + 1;
		}
		else
			high = mid - 1;
	}

	return ans;
}

int main()
{
	int x, y;
	std::cin >> x >> y;

	std::cout << onePercnet(x, y);

	return 0;
}
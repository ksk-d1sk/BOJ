// X보다 작은 수

#include <iostream>

int main()
{
	std::ios::sync_with_stdio(false);

	int N, X;
	std::cin >> N >> X;

	int* A = new int[N];

	for (int i = 0; i < N; i++)
	{
		std::cin >> A[i];
	}

	for (int i = 0; i < N; i++)
	{
		if (A[i] < X)
		{
			std::cout << A[i] << ' ';
		}
	}

	delete[] A;

	return 0;
}

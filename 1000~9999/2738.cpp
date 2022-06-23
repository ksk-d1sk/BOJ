#include <iostream>

int main()
{
	std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    
	int n, m;
	std::cin >> n >> m;
	
	int** a = new int*[n];
	for (int i = 0; i < n; i++)
		a[i] = new int[m];
	
	for (int i = 0; i < n; i++)
		for (int j = 0; j < m; j++)
			std::cin >> a[i][j];
	
	int temp;
	for (int i = 0; i < n; i++)
	{
		for (int j = 0; j < m; j++)
		{
			std::cin >> temp;
			std::cout << a[i][j] + temp << ' ';
		}
		std::cout << '\n';
	}
	
	for (int i = 0; i < n; i++)
		delete[] a[i];
		
	delete[] a;
	
	return 0;
}

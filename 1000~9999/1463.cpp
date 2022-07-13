// 1로 만들기

#include <iostream>
#include <algorithm>

using namespace std;

int main() {
	ios::sync_with_stdio(0);

	int* dp;
	int n;
	cin >> n;

	dp = new int[n+1];
	dp[1] = 0;

	for (int i = 2; i < n+1; i++) {
		dp[i] = dp[i-1] + 1;
		if (i % 2 == 0 && i % 3 == 0)
			dp[i] = min(min(dp[i], dp[i / 2] + 1), dp[i / 3] + 1);
		else if (i % 2 == 0)
			dp[i] = min(dp[i], dp[i/2] + 1);
		else if (i % 3 == 0)
			dp[i] = min(dp[i], dp[i/3] + 1);
	}

	cout << dp[n] << '\n';

	delete[] dp;

	return 0;
}
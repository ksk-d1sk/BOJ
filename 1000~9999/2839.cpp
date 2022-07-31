// 설탕 배달

#include <iostream>

using namespace std;

const int MX = 5001;
int dp[MX];

int main() {
	int n;
	cin >> n;

	dp[3] = 1;
	dp[5] = 1;

	for (int i = 6; i < MX; i++) {
		if (dp[i-3] + dp[i-5] != 0) {
			int a = 9999;
			int b = 9999;

			if (dp[i-3] != 0)
				a = dp[i-3];

			if (dp[i-5] != 0)
				a = dp[i-5];

			dp[i] = min(a, b) + 1;
		}
	}

	if (dp[n]) cout << dp[n];
	else cout << -1;

	return 0;
}
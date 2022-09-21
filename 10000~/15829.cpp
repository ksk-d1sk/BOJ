// Hashing

#include <iostream>

using namespace std;

int main() {
    short l;
    string str;
    long long sum = 0;
    short r = 31;
    int m = 1234567891;

    cin >> l;
    cin >> str;

    for (int i = 0; i < l; i++) {
        long long pow = 1;
        for (int j = 0; j < i; j++)
            pow = (pow * r) % m;
        sum = (sum + (str[i] - 96) * pow) % m;
    }
    
    cout << sum;

    return 0;
}

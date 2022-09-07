// 영수증

#include <iostream>

using namespace std;

int main() {
    int x;
    short n;
    int a;
    short b;
    
    cin >> x;
    cin >> n;
    while (n--) {
        cin >> a >> b;
        x -= a * b;
    }
    
    cout << (x ? "No" : "Yes");
}

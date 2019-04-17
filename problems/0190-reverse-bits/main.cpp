#include <bits/stdc++.h>

using namespace std;

class Solution {
public:
    // SWAR algorithm
    uint32_t reverseBits(uint32_t n) {
        n = ((n & 0xaaaaaaaa) >> 1) | ((n & 0x55555555) << 1);
        n = ((n & 0xcccccccc) >> 2) | ((n & 0x33333333) << 2);
        n = ((n & 0xf0f0f0f0) >> 4) | ((n & 0x0f0f0f0f) << 4);
        n = ((n & 0xff00ff00) >> 8) | ((n & 0x00ff00ff) << 8);
        return (n >> 16) | (n << 16);
    }
    // uint32_t reverseBits(uint32_t n) {
    //     uint32_t answer = 0;
    //     for (int i = 0; i < 32; ++i) {
    //         answer <<= 1;
    //         answer += n & 1;
    //         n >>= 1;
    //     }
    //     return answer;
    // }
};

int main() {
    uint32_t ret = Solution().reverseBits(0b00000010100101000001111010011100);

    string out = to_string(ret);
    cout << out << endl;
}

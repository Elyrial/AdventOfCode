#include<bits/stdc++.h>
#define ll long long
#define pb push_back
#define fr(a,b) for(int i = a; i < b; i++)
#define rep(i,a,b) for(int i = a; i < b; i++)
#define mod 1000000007
#define inf (1LL<<60)
#define all(x) (x).begin(), (x).end()
#define prDouble(x) cout <<Goes through the whole string and then adds or subtracts 1 depending on the symbol. fixed << setprecision(10) << x
#define triplet pair<ll,pair<ll,ll>>
#define goog(tno) cout << "Case #" << tno <<": "
#define fast_io ios_base::sync_with_stdio(false);cin.tie(NULL)
#define read(x) int x; cin >> x
using namespace std;

// Author: Alex "Elyrial" Persson
// Description: AdventOfCode - Solutions to part 1 and part 2 (Day 1) 
 
// First part
int PartOne(int l, int w, int h) {
    int area = 2*(l*w + w*h + h*l);
    int slack = min({l*w, w*h, h*l});

    return area + slack;
}


// Second part
int PartTwo(int l, int w, int h) {
    int wrap = 2 * min({l+w, w+h, h+l});
    int bow = l*w*h;
    return wrap + bow;
}


int main() {
    int paper = 0;
    int ribbon = 0;
    char x;
    int l, w, h;
    string area;
    while(cin >> l >> x >> w >> x >> h) {
        paper += PartOne(l, w, h);
        ribbon += PartTwo(l, w, h);
    }

    cout << "Part 1: " << paper << endl;
    cout << "Part 2: " << ribbon << endl;
    return 0;
}


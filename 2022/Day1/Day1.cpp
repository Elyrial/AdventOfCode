#include<bits/stdc++.h>
#define ll long long
#define pb push_back
#define fr(a,b) for(int i = a; i < b; i++)
#define rep(i,a,b) for(int i = a; i < b; i++)
#define mod 1000000007
#define inf (1LL<<60)
#define all(x) (x).begin(), (x).end()
#define prDouble(x) cout << fixed << setprecision(10) << x
#define triplet pair<ll,pair<ll,ll>>
#define goog(tno) cout << "Case #" << tno <<": "
#define fast_io ios_base::sync_with_stdio(false);cin.tie(NULL)
#define read(x) int x; cin >> x
using namespace std;


// Author: Alex "Elyrial" Persson

void partTwo(vector<int> V) {
    sort(V.begin(), V.end(), greater<int>());
    int top3 = 0;
    for(int i = 0; i < 3; i++) {
        top3 += V[i];
    }
        
    cout << "Part 2: " << top3 << endl; 

}


void partOne(vector<int> V) {
    cout << "Part 1: " << *max_element(V.begin(), V.end()) << endl;
}
   

int main() {
    vector<int> V;
    int cal = 0;
    string input;
    while(getline(cin, input)) {
        if(input.empty()) {
             V.pb(cal);
             cal = 0;
        } else {
            cal += stoi(input);
        }
    }
    partOne(V);
    partTwo(V);
    return 0;
}


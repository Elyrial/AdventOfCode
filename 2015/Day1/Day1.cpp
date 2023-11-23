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
void PartOne(string seq) {
    int floor = 0;
    // Goes through the whole string and then adds or subtracts 1 from floor depending on the symbol.
    for(int i = 0; i < seq.length(); i++) {
        if(seq[i] == '(') {
            floor++;
        } else if (seq[i] == ')') {
            floor--;
        }
    }
    // Prints out result
    cout << "Part 1: " << floor << endl;
}


// Second part
void PartTwo(string seq) {
    int floor = 0;
    int basement;
    // Same algorithm from part 1 
    for(int i = 0; i < seq.length(); i++) {
        if(seq[i] == '(') {
            floor++;
        } else if(seq[i] == ')') {
            floor--;
        }
        // Breaking the loop once floor = -1
        if(floor == -1) {
            basement = i;
            break;
        }
    }
    // Adds one because of indexing (First character = 1st position)
    cout << "Part 2: " << basement + 1 << endl;
}

int main() {
    // Reading the string@s
    string seq;
    cin >> seq;
    // Run the functions
    PartOne(seq);
    PartTwo(seq);
    return 0;
}


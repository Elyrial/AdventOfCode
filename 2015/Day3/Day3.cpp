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


int PartTwo(const string& seq) {
    int x = 0;
    int y = 0;
   
    int rX = 0;
    int rY = 0;

    set<pair<int, int>> visits;


    visits.insert({x, y});


    for (int i = 0; i < seq.size(); ++i) {
        char direction = seq[i];

        if (i % 2 == 0) {
            if (direction == '>') {
                x++;
            } else if (direction == '<') {
                x--;
            } else if (direction == '^') {
                y++;
            } else if (direction == 'v') {
                y--;
            }
            visits.insert({x, y});
        } else {
            if (direction == '>') {
                rX++;
            } else if (direction == '<') {
                rX--;
            } else if (direction == '^') {
                rY++;
            } else if (direction == 'v') {
                rY--;
            }
            visits.insert({rX, rY});
        }
    }

    return visits.size();
}

int PartOne(const string& seq) {
    int x = 0;
    int y = 0;

    set<pair<int, int>> visits;

    visits.insert({x, y});

    for (char direction : seq) {
        if (direction == '>') {
            x++;
        } else if (direction == '<') {
            x--;
        } else if (direction == '^') {
            y++;
        } else if (direction == 'v') {
            y--;
        }

        visits.insert({x, y});
    }

    return visits.size();
}



int main() {
    string seq;
    cin >> seq;
    
    cout << "Part 1: " << PartOne(seq) << endl;
    cout << "Part 2: " << PartTwo(seq) << endl;
    return 0;
}

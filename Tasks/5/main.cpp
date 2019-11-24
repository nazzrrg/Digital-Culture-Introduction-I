#include <iostream>
#include <vector>
#include <map>

using namespace std;

typedef map<int, vector<int>*> AdjacencyList;
typedef vector<pair<int, int>> EdgesList;

AdjacencyList get_adjacency_list(const EdgesList& input) {
    AdjacencyList result;

    for (auto it : input) {
        if (result.find(it.first) == result.end()) {

        } else {
            
        }
    }

    return result;
}

int main(int argc, char** argv) {
    freopen(argv[0], "r", stdin);
    freopen("output.txt", "w", stdout);

    EdgesList edges_list;
    int from, to;
    while (cin >> from >> to) {
        edges_list.emplace_back(from, to);
    }

    AdjacencyList adjacency_list = get_adjacency_list(edges_list);

    for (auto it : adjacency_list) {
        cout << it.first << ": {";
        for (auto item : it.second) {
            cout << item <<' ';
        }
        cout << "}\n";
    }



    return 0;
}
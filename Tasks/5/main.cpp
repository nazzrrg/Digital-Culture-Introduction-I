#include <iostream>
#include <vector>
#include <map>

using namespace std;

typedef map<int, vector<int>> AdjacencyList;
typedef vector<pair<int, int>> EdgesList;

AdjacencyList get_adjacency_list(const EdgesList& input) {
    map<int, vector<int>> result;

    for (auto it : input) {
        result.insert(it.first, it.second);
    }

    return result;
}

int main(int argc, char** argv) {
    freopen(argv[0], "r", stdin);

    EdgesList edges_list;
    int from, to;
    while (cin >> from >> to) {
        edges_list.emplace_back(from, to);
    }

    AdjacencyList adjacency_list = get_adjacency_list(edges_list);



    return 0;
}
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>

using namespace std;

bool is_possible(long long time, long long distance, long long pressed_for) {
    return ((time - pressed_for) * pressed_for) > distance;
}

long long binary_get_possibilities_count(long long time, long long distance) {
    long long middle = time / 2;
    long long minimum;
    long long step = (middle + 1)/2;
    long long test_middle = middle - step;
    while (step) {
        if (step != 1) {
            step = (step + 1)/2;
        } else {
            step = 0;
        }
        if (is_possible(time, distance, test_middle)) {
            test_middle -= step;
        } else {
            test_middle += step;
        }
    }
    if (is_possible(time, distance, test_middle)) {
        minimum = test_middle;
    } else {
        minimum = test_middle + 1;
    }

    long long maximum;
    step = (middle+1)/2;
    test_middle = middle + step;
    while (step) {
        if (step != 1) {
            step = (step + 1)/2;
        } else {
            step = 0;
        }
        if (is_possible(time, distance, test_middle)) {
            test_middle += step;
        } else {
            test_middle -= step;
        }
    }
    if (is_possible(time, distance, test_middle)) {
        maximum = test_middle + 1;
    } else {
        maximum = test_middle;
    }

    return maximum - minimum;
}

long long NumDigits(long long n) {
    long long digits = 0;
    while (n) {
        n /= 10;
        ++digits;
    }
    return digits;
}

long long power(long long base, long long n) {
    long long res = base;
    while (--n) {
        res *= base;
    }
    return res;
}

int main() {
    ifstream file("input.txt");
    string line;
    getline(file, line);
    istringstream splitter(line);
    string dummy;
    splitter >> dummy;
    long long token;
    vector<long long> times;
    while (splitter >> token) {
        times.push_back(token);
    }

    getline(file, line);
    splitter = istringstream(line);
    splitter >> dummy;
    vector<long long> distances;
    while (splitter >> token) {
        distances.push_back(token);
    }

    long long possibilities;
    long long total = 1;
    long long bigtime = 0;
    long long bigdistance = 0;

    for (long long i = 0; i < times.size(); i++) {
        bigtime = bigtime * power(10, NumDigits(times[i])) + times[i];
        bigdistance = bigdistance * power(10, NumDigits(distances[i])) + distances[i];
        possibilities = binary_get_possibilities_count(times[i], distances[i]);
        cout << times[i] << " >> " << distances[i] << " >> " << possibilities << endl;
        total *= possibilities;
    }

    long long bigpossibilities = binary_get_possibilities_count(bigtime, bigdistance);
    cout << bigtime << " >> " << bigdistance << " >> " << bigpossibilities << endl;

    cout << total << endl;
}

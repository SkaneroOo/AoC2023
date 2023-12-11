using Microsoft.VisualBasic;

var file = "input.txt";

if (!File.Exists(file)) {
    Console.WriteLine("File not found");
    Environment.Exit(1);
}

List<(string, int)> hands = new List<(string, int)>();
Dictionary<string, int> points = new Dictionary<string, int>();
Dictionary<string, int> new_points = new Dictionary<string, int>();
List<char> order = new List<char>{ 'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2' };
order.Reverse();

foreach (var line in File.ReadLines(file)) {
    var temp = line.Split(' ');
    var cards = temp[0];
    var bet = temp[1];

    hands.Add((cards, int.Parse(bet)));

    points.Add(cards, CalculatePoints(cards));
    new_points.Add(cards, RecalculatePoints(cards));

}

hands.Sort(delegate((string, int) x, (string, int) y) {
    var card1 = x.Item1;
    var card2 = y.Item1;
    if (points[card1] > points[card2]) {
        return 1;
    }
    if (points[card1] < points[card2]) {
        return -1;
    }
    for (int i = 0; i < card1.Length; i++) {
        if (order.IndexOf(card1[i]) > order.IndexOf(card2[i])) {
            return 1;
        }
        if (order.IndexOf(card1[i]) < order.IndexOf(card2[i])) {
            return -1;
        }
    }
    return 0;
});

var total = 0;

for (int i = 0; i < hands.Count; i++) {
    total += hands[i].Item2 * (i+1);
}

Console.WriteLine(total);

order.Remove('J');
order.Insert(0, 'J');

hands.Sort(delegate((string, int) x, (string, int) y) {
    var card1 = x.Item1;
    var card2 = y.Item1;
    if (new_points[card1] > new_points[card2]) {
        return 1;
    }
    if (new_points[card1] < new_points[card2]) {
        return -1;
    }
    for (int i = 0; i < card1.Length; i++) {
        if (order.IndexOf(card1[i]) > order.IndexOf(card2[i])) {
            return 1;
        }
        if (order.IndexOf(card1[i]) < order.IndexOf(card2[i])) {
            return -1;
        }
    }
    return 0;
});

var new_total = 0;

for (int i = 0; i < hands.Count; i++) {
    new_total += hands[i].Item2 * (i+1);
}

Console.WriteLine(new_total);

int CalculatePoints(string cards) {
    var card_counter = new Dictionary<char, int>();
    foreach (var card in cards) {
        if (card_counter.ContainsKey(card)) {
            card_counter[card]++;
        } else {
            card_counter.Add(card, 1);
        }
    }
    switch (card_counter.Count) {
        case 5:                 // High card
            return 0;
        case 4:                 // One pair
            return 1;
        case 3:                 // Two pair | Three of a kind
            foreach (var num in card_counter.Values) {
                if (num == 3) {
                    return 3;   // Three of a kind
                }
            }
            return 2;           // Two pair
        case 2:                 // Full house | Four of a kind
            foreach (var num in card_counter.Values) {
                if (num == 4) {
                    return 5;   // Four of a kind
                }
            }
            return 4;           // Full house
        case 1:                 // Five of a kind
            return 6;
        default:                // Unreachable
            return -1;
    }
}

int RecalculatePoints(string cards) {
    var card_counter = new Dictionary<char, int>();
    foreach (var card in cards) {
        if (card_counter.ContainsKey(card)) {
            card_counter[card]++;
        } else {
            card_counter.Add(card, 1);
        }
    }

    if (card_counter.ContainsKey('J') && card_counter.Count > 1) {
        var jokers = card_counter['J'];
        card_counter.Remove('J');
        
        var max = card_counter.MaxBy(x => x.Value).Key;

        card_counter[max] += jokers;
    }

    switch (card_counter.Count) {
        case 5:                 // High card
            return 0;
        case 4:                 // One pair
            return 1;
        case 3:                 // Two pair | Three of a kind
            foreach (var num in card_counter.Values) {
                if (num == 3) {
                    return 3;   // Three of a kind
                }
            }
            return 2;           // Two pair
        case 2:                 // Full house | Four of a kind
            foreach (var num in card_counter.Values) {
                if (num == 4) {
                    return 5;   // Four of a kind
                }
            }
            return 4;           // Full house
        case 1:                 // Five of a kind
            return 6;
        default:                // Unreachable
            return -1;
    }
}
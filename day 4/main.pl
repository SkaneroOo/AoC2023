use strict;
use warnings;

my $file = 'input.txt';
open my $info, $file or die "Could not open $file: $!";

my $total = 0;
my $scratchcards = 0;

my %cards = (1 => 1);

while( my $line = <$info>)  {
    if (!exists($cards{$.})) {
        $cards{$.} = 1;
    }
    ($_, my $data) = split /: +/, $line;
    my ($my, $winning) = split(/ +\| +/, $data);
    my @my = split / +/, $my;
    my @winning = split / +/, $winning;
    my $this = 0;
    my $matches = 0;
    for (my $i = 0; $i < @my; $i++) {
        $my[$i] += 0;
        for (my $j = 0; $j < @winning; $j++) {
            $winning[$j] += 0;
            if ($my[$i] eq $winning[$j]) {
                $matches++;
                if ($this == 0) {
                    $this = 1;
                } else {
                    $this *= 2;
                }
            }
        }
    }

    for (my $i = 0; $i < $matches; $i++) {
        if (exists $cards{$. + $i + 1}) {
            $cards{$. + $i + 1} += $cards{$.};
        }
        else {
            $cards{$. + $i + 1} = $cards{$.} + 1;
        }
    }

    $total += $this;
    $scratchcards += $cards{$.}
}


print "$total\n";
print "$scratchcards\n";
close $info;
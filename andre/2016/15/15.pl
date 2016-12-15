#!/usr/bin/env perl

use strict;
use warnings;

my @discs;

# Try each time step until solved
sub solve {
    my $offset = 0;

    while (1) {
        my $sum = 0;
        for my $i (0..scalar(@discs)-1) {
            my $mod = $discs[$i][0];
            my $val = $discs[$i][1];
            $sum += ($i + 1 + $val + $offset) % $mod;
        }
        last if ($sum == 0);
        $offset++;
    }
    return $offset;
}

while (<>) {
    push @discs, [$1, $2] if ($_ =~ m/Disc #\d has (\d+) positions; at time=0, it is at position (\d+)/);
}

print solve(), "\n";
push @discs, [11, 0];
print solve(), "\n";

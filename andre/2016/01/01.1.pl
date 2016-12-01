#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    chomp $_;
    my ($x, $y) = (0, 0);

    # 0 - north
    # 1 - east
    # 2 - south
    # 3 - west
    my $facing = 0;

    while ($_ =~ m/([LR])([0-9]+),?/g) {
        # Update direction
        if ($1 eq "R") {
            $facing = ($facing + 1) % 4;
        } else {
            $facing = ($facing + 3) % 4;
        }

        if ($facing == 0) {
            $y += $2;
        } elsif ($facing == 1) {
            $x += $2;
        } elsif ($facing == 2) {
            $y -= $2;
        } elsif ($facing == 3) {
            $x -= $2;
        }
    }
    print abs($x) + abs($y), "\n";
}

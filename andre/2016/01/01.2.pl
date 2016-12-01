#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    chomp $_;
    my %visited;
    my ($x, $y) = (0, 0);
    $visited{"0,0"} = 1;

    # 0 - north
    # 1 - east
    # 2 - south
    # 3 - west
    my $facing = 0;

    while ($_ =~ m/([A-Z])([0-9]+),?/g) {
        my $visited_found = 0;

        # Update direction
        if ($1 eq "R") {
            $facing = ($facing + 1) % 4;
        } else {
            $facing = ($facing + 3) % 4;
        }

        foreach my $i (1..$2) {
            if ($facing == 0) {
                $y++;
            } elsif ($facing == 1) {
                $x++;
            } elsif ($facing == 2) {
                $y--;
            } elsif ($facing == 3) {
                $x--;
            }
            if (exists($visited{"$x,$y"})) {
                print abs($x) + abs($y), "\n";
                $visited_found = 1;
                last;
            }
            $visited{"$x,$y"} = 1;
        }
        if ($visited_found) {
            last;
        }
    }
}

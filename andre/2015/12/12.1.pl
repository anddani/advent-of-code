#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    chomp $_;
    my @numbers;
    while ($_ =~ m/(-?\d+)/g) {
        push @numbers, $1;
    }

    my $sum = 0;
    map { $sum += $_ } @numbers;
    print "$sum\n";
}

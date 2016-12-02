#!/usr/bin/env perl

use strict;
use warnings;

use List::Util qw/min max/;

my @dial = (
    ["1", "2", "3"],
    ["4", "5", "6"],
    ["7", "8", "9"]
);

my $result = "";

my ($row, $col) = (1, 1);
while (<>) {
    chomp $_;
    while (m/([LURD])/g) {
        if ($1 eq "L") {
            $col = max $col-1, 0;
        } elsif ($1 eq "U") {
            $row = max $row-1, 0;
        } elsif ($1 eq "R") {
            $col = min $col+1, 2;
        } elsif ($1 eq "D") {
            $row = min $row+1, 2;
        }
    }
    $result .= $dial[$row][$col];
}
print "$result\n";

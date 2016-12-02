#!/usr/bin/env perl

use strict;
use warnings;

use List::Util qw/min max/;

my @keypad = (
    [ "",  "", "1",  "",  ""],
    [ "", "2", "3", "4",  ""],
    ["5", "6", "7", "8", "9"],
    [ "", "A", "B", "C",  ""],
    [ "",  "", "D",  "",  ""]
);
my $result = "";

my ($row, $col) = (2, 0);

while (<>) {
    chomp $_;
    while (m/([LURD])/g) {
        if ($1 eq "L") {
            my $new = $col-1;
            if ($col-1 >= 0 and $keypad[$row][$col-1] ne "") {
                $col--;
            }
        } elsif ($1 eq "U") {
            if ($row-1 >= 0 and $keypad[$row-1][$col] ne "") {
                $row--;
            }
        } elsif ($1 eq "R") {
            if ($col+1 <= 4 and $keypad[$row][$col+1] ne "") {
                $col++;
            }
        } elsif ($1 eq "D") {
            if ($row+1 <= 4 and $keypad[$row+1][$col] ne "") {
                $row++;
            }
        }
    }
    $result .= $keypad[$row][$col];
}
print "$result\n";

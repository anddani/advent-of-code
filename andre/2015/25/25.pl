#!/usr/bin/env perl

use strict;
use warnings;

my $row;
my $col;

while (<>) {
    chomp $_;
    if ($_ =~ m/row (\d+), column (\d+)/) {
        $row = int($1);
        $col = int($2);
    }
}

my $prev = 20151125;
my $mul = 252533;
my $mod = 33554393;
my ($c, $r) = (1, 1);

while (1) {
    if ($r == 1) {
        $r = $c + 1;
        $c = 1
    } else {
        $r--;
        $c++;
    }
    $prev = ($prev * $mul) % $mod;
    last if ($c == $col and $r == $row);
}
print "Sol: $prev\n";

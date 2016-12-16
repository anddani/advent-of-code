#!/usr/bin/env perl

use strict;
use warnings;

my $line;

sub dragon_curve {
    my $a = shift;
    my $b = reverse($a);
    $b =~ s/1/ /g;
    $b =~ s/0/1/g;
    $b =~ s/ /0/g;

    return $a . "0" . $b;
}

sub solve {
    my $num = shift;
    my $l = shift;
    $l = dragon_curve($l) while (length($l) < $num);
    $l = substr($l, 0, $num);
    while (length($l) % 2 == 0) {
        my $temp = $l;
        $l = "";
        $l .= (($1 eq $2) ? "1" : "0") while ($temp =~ m/(\d)(\d)/g);
    }
    return $l;
}

while (<>) {
    $line = $_;
    chomp $line;
}

print "Part 1: ", solve(272, $line), "\n";
print "Part 2: ", solve(35651584, $line), "\n";

#!/usr/bin/env perl

use strict;
use warnings;
use Data::Dumper;
use Math::Prime::Util qw/fordivisors/;

my $num = 3400000;

sub part1 {
    for my $i (1..$num) {
        my $tot = 0;
        fordivisors { $tot += $_ } $i;
        return $i if ($tot >= $num);
    }
}

sub part2 {
    my @h;

    for (my $e = 1; $e < 100000; $e++) {
        for my $i (1..50) {
            $h[$i*$e] += $e*11;
        }
    }

    for my $i (1..scalar(@h)-1) {
        next if (not exists($h[$i]));
        if ($h[$i] >= $num) {
            return $i * 10;
        }
    }
}

print "Part 1: ", part1(), "\n";
print "Part 2: ", part2(), "\n";

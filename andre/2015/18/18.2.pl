#!/usr/bin/env perl

use strict;
use warnings;

use Data::Dumper;

my @offset = ([-1, -1], [-1, 0], [-1, 1], [0, 1], [1, 1], [1, 0], [1, -1], [0, -1]);

my @lights;

my $rows = 0;
my $cols;

sub solve {
    my @new_lights;


    $lights[0][0] = 1;
    $lights[0][$cols-1] = 1;
    $lights[$rows-1][0] = 1;
    $lights[$rows-1][$cols-1] = 1;

    for my $row (0..$rows-1) {
        for my $col (0..$cols-1) {
            my $num_neighbors = 0;

            # Count neighbors
            my @possible = grep { $row + ${$_}[0] >= 0 and $row + ${$_}[0] < $rows and
                                  $col + ${$_}[1] >= 0 and $col + ${$_}[1] < $cols } @offset;

            for my $p (@possible) {
                $num_neighbors++ if ($lights[$row + $p->[0]][$col + $p->[1]]);
            }

            if ($lights[$row][$col]) {
                $new_lights[$row][$col] = ($num_neighbors == 2 or $num_neighbors == 3);
            } else {
                $new_lights[$row][$col] = ($num_neighbors == 3);
            }
        }
    }

    $new_lights[0][0] = 1;
    $new_lights[0][$cols-1] = 1;
    $new_lights[$rows-1][0] = 1;
    $new_lights[$rows-1][$cols-1] = 1;

    @lights = @new_lights;
}

while (<>) {
    chomp $_;
    my @row = map { $_ eq '#' ? 1 : 0 } split(//, $_);
    $lights[$rows] = \@row;
    $cols = scalar(@row);
    $rows++;
}

print_state();
for (1..100) {
    solve();
}

my $part2 = 0;
map { map { $part2 += $_ } @{$_} } @lights;
print "$part2\n";

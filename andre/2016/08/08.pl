#!/usr/bin/env perl

use strict;
use warnings;

use Data::Dumper;

my @screen;
push @screen, [(0) x 50] for 1..6;
my $num_rows = scalar(@screen);

while (<>) {
    chomp $_;

    # Fill rectangle with NxM lights
    if ($_ =~ m/rect (\d+)x(\d+)/) {
        for my $x (0..($1-1)) {
            for my $y (0..($2-1)) {
                $screen[$y]->[$x] = 1;
            }
        }
    # Rotate column or row
    } elsif ($_ =~ m/rotate.*(\w)=(\d+) by (\d+)/) {
        if ($1 eq "y") {
            # Rotate row
            unshift @{$screen[$2]}, pop @{$screen[$2]} for 1..$3;
        } elsif ($1 eq "x") {
            # Rotate column
            for (1..$3) {
                my $temp = $screen[$num_rows-1]->[$2];
                for my $i (reverse 1..($num_rows-1)) {
                    $screen[$i]->[$2] = $screen[$i-1]->[$2];
                }
                $screen[0]->[$2] = $temp;
            }
        }
    }
}

my $sum = 0;
map { $sum += scalar(grep { $_ } @{$_}) } @screen;
print "$sum\n";

for my $row (0..5) {
    for my $col (0..49) {
        print $screen[$row]->[$col] ? '#' : '.';
    }
    print "\n";
}

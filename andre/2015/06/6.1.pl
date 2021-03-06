#!/usr/bin/env perl

use strict;
use warnings;

my %lights;

# Initialize lights to zero
for (my $i = 0; $i < 1000; $i++) {
    for (my $j = 0; $j < 1000; $j++) {
        $lights{$i.",".$j} = 0;
    }
}

while (<>) {
    chomp $_;
    if ($_ =~ m/([\w ]+) (\d+),(\d+)\D*(\d+),(\d+)/) {
        my $op = $1;
        my $from_x = $2;
        my $from_y = $3;
        my $to_x = $4;
        my $to_y = $5;

        for (my $i = $from_x; $i <= $to_x; $i++) {
            for (my $j = $from_y; $j <= $to_y; $j++) {
                if ($op eq "turn on") {
                    $lights{$i.",".$j} = 1;
                } elsif ($op eq "turn off") {
                    $lights{$i.",".$j} = 0;
                } elsif ($op eq "toggle") {
                    $lights{$i.",".$j} ^= 1;
                }
            }
        }
    }
}

my $ones = grep(/1/, values %lights);
print "$ones\n";

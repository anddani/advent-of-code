#!/usr/bin/env perl

use strict;
use warnings;

use Algorithm::Combinatorics qw/combinations/;
use Data::Dumper;

my @presents;

while (<>) {
    chomp $_;
    push @presents, int($_);
}

my $sum = 0;
map { $sum += $_ } @presents;

sub solve {
    my ($split) = shift;

    my $best_qe = -1;
    my $target = $sum/$split;
    my $picks = 1;

    while ($best_qe == -1) {
        my $permutor = combinations(\@presents, $picks);
        while (my $p = $permutor->next()) {
            my $g1_sum = 0;
            my $g1_qe = 1;
            map { $g1_sum += $_ } @{$p};
            map { $g1_qe *= $_ } @{$p};
            if ($g1_sum == $target and ($best_qe == -1 or $g1_qe < $best_qe)) {
                $best_qe = $g1_qe;
            }
        }
        $picks++;
    }
    return $best_qe;
}

print "Part 1: ", solve(3), "\n";
print "Part 2: ", solve(4), "\n";

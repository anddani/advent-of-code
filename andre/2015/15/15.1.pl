#!/usr/bin/env perl

use strict;
use warnings;

use List::Util qw/max min/;

my %ingredients;

while (<>) {
    chomp $_;
    $_ =~ m/(\w+):.* (-?\d+).* (-?\d+).* (-?\d+).* (-?\d+), /;

    my $cap = int($2);
    my $dur = int($3);
    my $fla = int($4);
    my $tex = int($5);

    $ingredients{$1} = {cap=>$cap, dur=>$dur, fla=>$fla, tex=>$tex};
}

my @keys = keys %ingredients;
my @properties = keys %{$ingredients{$keys[0]}};
my $best_score = 0;

for my $sugar (1..97) {
    for my $sprinkles (1..(98-$sugar)) {
        for my $candy (1..(99-$sugar-$sprinkles)) {
            my $chocolate = 100-$sugar-$sprinkles-$candy;
            my $prod = 1;

            for my $p (@properties) {
                my $sum = 0;
                $sum += $sugar * $ingredients{'Sugar'}{$p};
                $sum += $sprinkles * $ingredients{'Sprinkles'}{$p};
                $sum += $candy * $ingredients{'Candy'}{$p};
                $sum += $chocolate * $ingredients{'Chocolate'}{$p};
                $prod *= max($sum, 0);
            }
            $best_score = max $best_score, $prod;
        }
    }
}

print "$best_score\n";

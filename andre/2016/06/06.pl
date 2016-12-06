#!/usr/bin/env perl

use strict;
use warnings;

use Data::Dumper;

my $frequencies = {};

while (<>) {
    chomp $_;

    my $message = $_;
    for my $i (0..7) {
        my $char = substr($message, $i, 1);
        $frequencies->{$i}{$char}++;
    }
}

my $part1 = "";
my $part2 = "";
for my $i (0..7) {
    my $hash = $frequencies->{$i};
    my @best = sort {
        $frequencies->{$i}{$b} <=> $frequencies->{$i}{$a}
    } keys %$hash;
    $part1 .= $best[0];
    $part2 .= $best[scalar(@best)-1];
}

print "$part1\n";
print "$part2\n";

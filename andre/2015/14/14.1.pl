#!/usr/bin/env perl

use strict;
use warnings;

use List::Util qw/max min/;

my @lengths;
my $seconds = 2503;

while (<>) {
    chomp $_;
    $_ =~ m/(\w+).* (\d+).* (\d+).* (\d+)/;
    my ($speed, $energy, $rest) = (int($2), int($3), int($4));

    my $tot_time = $energy + $rest;
    my $passed = int($seconds / $tot_time) * $speed*$energy;
    my $inside = $speed * min($energy, ($seconds % $tot_time));
    push @lengths, $passed+$inside;
}

my $best = max @lengths;
print "$best\n";

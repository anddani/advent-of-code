#!/usr/bin/env perl

use strict;
use warnings;

use List::Util qw/max min/;

my @lengths;
my %reindeers;
my $seconds = 2503;

sub current_length {
    my ($speed, $energy, $rest, $sec) = ($_[0], $_[1], $_[2], $_[3]);
    my $tot_time = $energy + $rest;
    my $passed = int($sec / $tot_time) * $speed*$energy;
    my $inside = $speed * min($energy, ($sec % $tot_time));

    return $passed+$inside;
}

while (<>) {
    chomp $_;
    $_ =~ m/(\w+).* (\d+).* (\d+).* (\d+)/;
    my ($speed, $energy, $rest) = (int($2), int($3), int($4));
    $reindeers{$1} = {speed=>$speed, energy=>$energy, rest=>$rest, points=>0};
}

my @keys = keys %reindeers;

# For each second, add one point to the leader
my $leader = "";
my $best_points = 0;
for my $sec (1..$seconds) {
    my $best_pos = 0;
    for my $reindeer (@keys) {
        my $s = $reindeers{$reindeer}{speed};
        my $e = $reindeers{$reindeer}{energy};
        my $r = $reindeers{$reindeer}{rest};

        my $pos = current_length($s, $e, $r, $sec);
        if ($pos > $best_pos) {
            $leader = $reindeer;
            $best_pos = $pos;
        }
    }
    $reindeers{$leader}{points}++;
    $best_points = max $best_points, $reindeers{$leader}{points};
}

print "$best_points\n";

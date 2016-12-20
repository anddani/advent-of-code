#!/usr/bin/env perl

use strict;
use warnings;

use List::Util qw/max/;

my @ranges;

while (<>) {
    chomp $_;
    $_ =~ m/(\d+)-(\d+)/;
    push @ranges, [int($1), int($2)];
}

my $largest = 4294967295;

my @sorted = sort { @{$a}[0] <=> @{$b}[0] } @ranges;

my $curr_e = 0;
for my $intervals (@sorted) {
    # Break when gap is found
    last if $curr_e < @{$intervals}[0];
    $curr_e = max($curr_e, @{$intervals}[1]+1);
}

print "Part 1: $curr_e\n";

$curr_e = 0;
my $sum = 0;
for my $intervals (@sorted) {
    # Add gap to sum
    $sum += @{$intervals}[0] - $curr_e if ($curr_e < @{$intervals}[0]);
    $curr_e = max($curr_e, @{$intervals}[1]+1);
}

print "Part 2: ", $largest + 1 + $sum - $curr_e, "\n";

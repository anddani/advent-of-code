#!/usr/bin/env perl

use strict;
use warnings;

use List::Permutor;
use List::Util qw/max min/;

my %distances;



while (<>) {
    chomp $_;
    $_ =~ m/(\w+) to (\w+) = (\d+)/;
    $distances{$1}{$2} = int($3);
    $distances{$2}{$1} = int($3);
}

my @cities = keys %distances;
my $permutor = List::Permutor->new (@cities);
my @trips;

while (my @p = $permutor->next()) {
    my $sum = 0;
    my $p_len = @p;
    for (my $i = 0; $i < $p_len-1; $i++) {
        $sum += $distances{$p[$i]}{$p[$i+1]};
    }
    push @trips, $sum;
}
my $min = min @trips;
my $max = max @trips;
print "$min\n";
print "$max\n";

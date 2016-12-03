#!/usr/bin/env perl

use strict;
use warnings;

use List::Permutor;
use List::Util qw/max min/;

my %happiness;

while (<>) {
    chomp $_;
    $_ =~ m/^(\w+).* (\w+) (\d+) .* (\w+)\./;
    my $to_add = ($2 eq 'gain') ? int($3) : -int($3);

    if (exists($happiness{$1}{$4})) {
        $happiness{$1}{$4} += $to_add;
    } else {
        $happiness{$1}{$4} = $to_add;
    }
}

my @people = keys %happiness;
foreach my $p (@people) {
    $happiness{"Me"}{$p} = 0;
    $happiness{$p}{"Me"} = 0;
}

my @keys = keys %happiness;
my $permutor = List::Permutor->new (@keys);
my @total;

while (my @p = $permutor->next()) {
    my $sum = 0;
    my $p_len = @p;
    for (my $i = 0; $i < $p_len-1; $i++) {
        $sum += $happiness{$p[$i]}{$p[$i+1]};
        $sum += $happiness{$p[$i+1]}{$p[$i]};
    }
    $sum += $happiness{$p[$p_len-1]}{$p[0]};
    $sum += $happiness{$p[0]}{$p[$p_len-1]};
    push @total, $sum;
}

my $max = max @total;
print "$max\n";

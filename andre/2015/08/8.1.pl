#!/usr/bin/env perl

use strict;
use warnings;

my $sum = 0;

while (<>) {
    chomp $_;
    my $orig_length = length($_);
    $_ =~ s/\\\\|\\"|\\x\w{2}/q/g;

    $sum += $orig_length - length($_) + 2;
}

print "$sum\n";

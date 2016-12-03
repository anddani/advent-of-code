#!/usr/bin/env perl

use strict;
use warnings;

my $num_valid = 0;

while (<>) {
    chomp $_;
    $_ =~ m/(\d+)\s*(\d+)\s*(\d+)/;
    $num_valid += ($1 + $2 > $3 and $1 + $3 > $2 and $2 + $3 > $1);
}
print "$num_valid\n";

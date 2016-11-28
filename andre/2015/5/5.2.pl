#!/usr/bin/env perl

use strict;
use warnings;

my $nice = 0;

while (<>) {
    chomp $_;
    if ($_ =~ m/(..).*\1/ &&
        $_ =~ m/(.).\1/) {
        $nice++;
    }
}

print "$nice\n";

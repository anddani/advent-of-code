#!/usr/bin/env perl

use strict;
use warnings;

my $sum = 0;
while (<>) {
    chomp $_;

    if ($_ =~ m/(.)(?!\1)(.)\2\1(?![^[]*\])/ and
        $_ =! m/\[\w*(.)(?!\1)(.)\2\1\w*\]/) {
        $sum++;
    }
}
print "$sum\n";

#!/usr/bin/env perl

use strict;
use warnings;

my $sum = 0;
while (<>) {
    chomp $_;

    if ($_ =~ m/\[\w*(.)(?!\1)(.)\1\w*\].*\2\1\2(?![^[]*\])/ or
        $_ =~ m/(.)(?!\1)(.)\1(?![^[]*\]).*\[\w*\2\1\2\w*\]/) {
        $sum++;
    }
}
print "$sum\n";

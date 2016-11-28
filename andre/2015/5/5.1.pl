#!/usr/bin/env perl

use strict;
use warnings;

my $nice = 0;

while (<>) {
    chomp $_;
    if ($_ =~ m/([aeiou].*){3}/ &&
        $_ =~ m/(.)\1/ &&
        $_ !~ m/ab|cd|pq|xy/) {
        $nice++;
    }
}

print "$nice\n";

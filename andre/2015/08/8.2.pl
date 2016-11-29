#!/usr/bin/env perl

use strict;
use warnings;

my $sum = 0;

while (<>) {
    chomp $_;
    my $orig_length = length($_);
    $_ =~ s/(["\\])/\\$1/g;
    $_ = "\"$_\"";

    $sum += length($_) - $orig_length;
}

print "$sum\n";

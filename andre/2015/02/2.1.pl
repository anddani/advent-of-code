#!/usr/bin/env perl

use strict;
use warnings;

my $paper = 0;

while (<>) {
    my @dims = split(/x/, $_);

    my ($l, $w, $h) = @dims;

    @dims = sort { $a <=> $b } @dims;

    $paper += 2*$l*$w + 2*$w*$h + 2*$h*$l + $dims[0]*$dims[1];
}

print $paper, "\n";

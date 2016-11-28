#!/usr/bin/env perl

use strict;
use warnings;

my $ribbon = 0;

while (<>) {
    my @dims = split(/x/, $_);

    my ($l, $w, $h) = @dims;
    @dims = sort { $a <=> $b } @dims;

    $ribbon += $l*$w*$h + 2*$dims[0] + 2*$dims[1];
}

print $ribbon, "\n";

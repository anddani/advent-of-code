#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    chomp $_;

    my %houses = ("0,0"=>1);
    my ($x, $y) = (0, 0);

    while ($_ =~ m/(.)/g) {
        if ($1 eq "^") {
            $y++;
        } elsif ($1 eq ">") {
            $x++;
        } elsif ($1 eq "v") {
            $y--;
        } elsif ($1 eq "<") {
            $x--;
        }
        $houses{"$x,$y"} = 1;
    }

    my $keys = keys %houses;
    print "$keys\n";
}

#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    chomp $_;

    my $santa = join("", ($_ =~ m/(.)./g));
    my $robo = join("", ($_ =~ m/.(.)/g));
    
    my %houses = ("0,0"=>1);
    my ($x, $y) = (0, 0);

    while ($santa =~ m/(.)/g) {
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

    ($x, $y) = (0, 0);

    while ($robo =~ m/(.)/g) {
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

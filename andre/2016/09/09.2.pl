#!/usr/bin/env perl

use strict;
use warnings;

sub part2 {
    my $line = $_[0];
    my $sum = 0;
    while ($line =~ m/(\((\d+)x(\d+)\))(.+)/g) {
        my $sub = $1;
        my $mul = $3;
        my $letters = substr($4, 0, $2);
        $sum += $mul * part2($letters);
        $line =~ s/\Q$sub$letters//;
    }
    return $sum + length($line);
}

while (<>) {
    chomp $_;
    print part2($_), "\n";
}

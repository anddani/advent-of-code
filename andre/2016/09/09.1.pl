#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    chomp $_;
    my $line = $_;
    my $sum = 0;
    while ($line =~ m/(\((\d+)x(\d+)\))(.+)/g) {
        my $letters = substr($4, 0, $2);
        $sum += length($letters x $3);
        $line =~ s/\Q$1$letters//;
    }
    $sum += length($line);
    print "$sum\n";
}

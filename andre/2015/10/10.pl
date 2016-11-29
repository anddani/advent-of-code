#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    chomp $_;
    my $line = $_;
    for (my $i = 0; $i < 40; $i++) {
        my $temp = "";
        while ($line =~ m/\G((\d)\2*)/g) {
            $temp .= length($1).$2;
        }
        $line = $temp;
    }

    print "40 times: ", length($line), "\n";

    for (my $i = 0; $i < 10; $i++) {
        my $temp = "";
        while ($line =~ m/\G((\d)\2*)/g) {
            $temp .= length($1).$2;
        }
        $line = $temp;
    }

    print "50 times: ", length($line), "\n";
}

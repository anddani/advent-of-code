#!/usr/bin/env perl

use strict;
use warnings;

use Digest::MD5 qw/md5_hex/;

while (<>) {
    chomp $_;
    my $num = 0;

    while (md5_hex($_.$num) !~ m/^00000.*/g) {
        $num++;
    }
    print "Five zeros: $num\n";

    while (md5_hex($_.$num) !~ m/^000000.*/g) {
        $num++;
    }
    print "Six zeros: $num\n";
}

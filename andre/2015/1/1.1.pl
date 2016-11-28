#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    my $l = () = $_ =~ m/\(/g;
    my $r = () = $_ =~ m/\)/g;
    print $l - $r, "\n"
}

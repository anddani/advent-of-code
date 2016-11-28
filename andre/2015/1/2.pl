#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    my $floor = 0;
    my $counter = 0;
    while ($_ =~ m/(.)/g) {
        $counter += 1;
        if ($1 eq "(") {
            $floor += 1;
        } else {
            $floor -= 1;
        }
        if ($floor == -1) {
            print $counter, "\n";
            last;
        }
    }
}

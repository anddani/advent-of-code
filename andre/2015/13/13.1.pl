#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    chomp $_;
    $_ =~ m/^(\w+).* (\d+) .* (\w+)\./;
    print "$1 $2 $3\n";
}

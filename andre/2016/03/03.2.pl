#!/usr/bin/env perl

use strict;
use warnings;

my $num_valid = 0;

my (@side1, @side2, @side3);

while (<>) {
    chomp $_;
    $_ =~ m/(\d+)\s*(\d+)\s*(\d+)/;
    push @side1, $1;
    push @side2, $2;
    push @side3, $3;

    if (scalar @side1 == 3) {
        $num_valid += valid_triangle(@side1);
        $num_valid += valid_triangle(@side2);
        $num_valid += valid_triangle(@side3);
        @side1 = ();
        @side2 = ();
        @side3 = ();
    }
}

sub valid_triangle {
    my ($a, $b, $c) = @_;
    return ($a + $b > $c and $a + $c > $b and $b + $c > $a);
}
print "$num_valid\n";

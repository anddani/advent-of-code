#!/usr/bin/env perl

use strict;
use warnings;

use JSON qw/decode_json/;

my @numbers;

while (<>) {
    chomp $_;

    my $json = decode_json $_;

    add_numbers($json);

    my $sum = 0;
    map { $sum += $_ } @numbers;
    print "$sum\n";
}

sub add_numbers {
    my $curr = shift @_;
    my $ref = ref($curr);

    if ($ref eq "ARRAY") {
        for my $v (@$curr) {
            if (ref($v)) {
                add_numbers($v);
            } elsif ($v =~ m/-?\d+/) {
                push @numbers, $v;
            }
        }
    } elsif ($ref eq "HASH") {
        for my $v (values(%$curr)) {
            if ($v eq "red") {
                return;
            }
        }
        for my $v (values(%$curr)) {
            if (ref($v)) {
                add_numbers($v);
            } elsif ($v =~ m/-?\d+/) {
                push @numbers, $v;
            }
        }
    }
}

#!/usr/bin/env perl

use strict;
use warnings;

use Data::Dumper;

my $sum = 0;

while (<>) {
    my %frequencies;
    chomp $_;
    $_ =~ m/([a-z\-]+)(\d+)\[(\w+)\]/;
    my $name = $1;
    my $id = int($2);
    my $checksum = $3;

    $name =~ s/-//g;
    my @name = split //, $name;

    for my $char (@name) {
        if (exists($frequencies{$char})) {
            $frequencies{$char}++;
        } else {
            $frequencies{$char} = 1;
        }
    }

    my @largest = sort {
        $frequencies{$b} <=> $frequencies{$a}
                        ||
                    $a cmp $b
    } keys %frequencies;

    if (join('', @largest[0..4]) eq $checksum) {
        $sum += $id;
    }
}

print "$sum\n"

#!/usr/bin/env perl

use strict;
use warnings;
use Data::Dumper;

my %things = {
    children=>3,
    cats=>7,
    samoyeds=>2,
    pomeranians=>3,
    akitas=>0,
    vizslas=>0,
    goldfish=>5,
    trees=>3,
    cars=>2,
    perfumes=>1
};

my %sues;

sub get_score {

}

while (<>) {
    chomp $_;
    $_ =~ m/(\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)/;

    print "$1 $2 $3 $4 $5 $6 $7\n";
    $sues{$1} = {$2=>$3, $4=>$5, $6=>$7}
}

print Dumper(\%sues);

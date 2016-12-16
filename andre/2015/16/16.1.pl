#!/usr/bin/env perl

use strict;
use warnings;
use Data::Dumper;

my $things = {
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


my @sues;

while (<>) {
    chomp $_;
    $_ =~ m/(\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)/;

    push @sues, {$2=>$3, $4=>$5, $6=>$7};
}

sub part1 {
    for my $i (0..scalar(@sues)-1) {
        my $sue = $sues[$i];
        my $bad = 0;
        for my $thing (keys(%{$things})) {
            if (exists($sue->{$thing}) and $sue->{$thing} ne $things->{$thing}) {
                $bad = 1;
                last;
            }
        }
        next if ($bad);
        return $i+1;
    }
}

sub part2 {
    for my $i (0..scalar(@sues)-1) {
        my $sue = $sues[$i];
        my $bad = 0;
        for my $thing (keys(%{$things})) {
            if (exists($sue->{$thing})) {
                if ($thing eq 'trees' or $thing eq 'cats') {
                    $bad = 1 unless $things->{$thing} < $sue->{$thing};
                } elsif ($thing eq 'goldfish' or $thing eq 'pomeranians') {
                    $bad = 1 unless $things->{$thing} > $sue->{$thing};
                } else {
                    $bad = 1 unless $things->{$thing} == $sue->{$thing};
                }
            }
        }
        next if ($bad);
        return $i+1;
    }
}
print part1(), "\n";
print part2(), "\n";

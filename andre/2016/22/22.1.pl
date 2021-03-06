#!/usr/bin/env perl

use strict;
use warnings;

use Data::Dumper;

my %nodes;
my $largest_x = 0;
while (<>) {
    chomp $_;
    if ($_ =~ m/node-x(\d+)-y(\d+)\s*(\d+)T\s*(\d+)T\s*(\d+)T\s*(\d+)/) {
        print "$1 $2 $3 $4 $5 $6\n";
        $nodes{"$1,$2"} = { size=>$3, used=>$4, avail=>$5, p=>$6 };
        $largest_x = $1 if ($1 > $largest_x);
    }
}

my $viable = 0;

for my $a (keys %nodes) {
    for my $b (keys %nodes) {
        if ($a ne $b and
            $nodes{$a}{used} > 0 and
            $nodes{$a}{used} <= $nodes{$b}{avail}) {
            $viable++;
        }
    }
}

print "Part 1: $viable\n";

my @queue;
push @queue, [0, "$largest_x,0"];

my $steps;
my $curr;
while (scalar(@queue)) {
    ($steps, $curr) = @{shift @queue};
    my ($x, $y) = split /,/, $curr;
    my $up = $x .",". ($y-1);
    my $down = $x .",". ($y+1);
    my $left = ($x-1) .",". $y;
    my $right = ($x+1) .",". $y;

    for my $dir ($up, $down, $left, $right) {
        if (exists($nodes{$dir})) {
            if ($nodes{$curr}{used} <= $nodes{$dir}{avail}) {

            }
        }
    }
}

print "Part 2: $steps\n";

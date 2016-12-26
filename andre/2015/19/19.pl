#!/usr/bin/env perl

use strict;
use warnings;

my $molecule;
my @replacements;

while (<>) {
    chomp $_;
    if ($_ =~ m/(\w+) => (\w+)/) {
        push @replacements, [$1, $2];
    } elsif ($_ =~ m/\w/) {
        $molecule = $_;
    }
}

sub part1 {
    my $mol = shift;
    my %molecules;
    for my $repl (@replacements) {
        while ($mol =~ m/$repl->[0]/g) {
            my $m = $mol;
            substr($m, $-[0], length($repl->[0])) = $repl->[1];
            $molecules{$m} = 1;
        }
    }
    return scalar(keys %molecules);
}

sub part2 {
    my $mol = shift;
    my $counter = 0;
    while ($mol ne "e") {
        for my $repl (@replacements) {
            $counter++ if ($mol =~ s/$repl->[1]/$repl->[0]/);
        }
    }
    return $counter;
}

print "Part 1: ", part1($molecule), "\n";
print "Part 2: ", part2($molecule), "\n";

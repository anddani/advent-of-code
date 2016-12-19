#!/usr/bin/env perl

use strict;
use warnings;

use Data::Dumper;

my @ring;

sub solve {
    my $num_elves = scalar(@ring);
    my $curr_elf = 0;
    my $debug = 0;

    my $best_elf = 0;
    my $best_pres = 1;

    while ($best_pres < $num_elves) {
        my $next = ($curr_elf+1) % $num_elves;
        $next = ($next + 1) % $num_elves while ($ring[$next] == 0);
        if ($ring[$curr_elf] > 0) {
            # Take all presents to left
            $ring[$curr_elf] += $ring[$next];
            $ring[$next] = 0;
            if ($ring[$curr_elf] > $best_pres) {
                $best_pres = $ring[$curr_elf];
                $best_elf = $curr_elf;
            }
        }
        $curr_elf = $next;
    }
    return $best_elf + 1;
}

while (<>) {
    chomp $_;
    push @ring, 1 for (1..$_);
}

print solve(), "\n";

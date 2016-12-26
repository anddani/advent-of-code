#!/usr/bin/env perl

use strict;
use warnings;
use Data::Dumper;
use List::Util qw/min max/;

my $boss;

my @armors = ([13, 0, 1], [31, 0, 2], [53, 0, 3], [75, 0, 4], [102, 0, 5], [0, 0, 0]);
my @weapons = ([8, 4, 0], [10, 5, 0], [25, 6, 0], [40, 7, 0], [74, 8, 0]);
my @rings = ([25, 1, 0], [50, 2, 0], [100, 3, 0], [20, 0, 1], [40, 0, 2], [80, 0, 3], [0, 0, 0], [0, 0, 0]);

while (<>) {
    chomp $_;
    if ($_ =~ m/Hit Points: (\d+)/) {
        $boss->{hp} = int($1);
    } elsif ($_ =~ m/Damage: (\d+)/) {
        $boss->{damage} = int($1);
    } elsif ($_ =~ m/Armor: (\d+)/) {
        $boss->{armor} = int($1);
    }
}

my @wins;
my @losses;

for my $weapon (@weapons) {
    for my $armor (@armors) {
        for my $r1 (0..scalar(@rings)-1) {
            for my $r2 (0..$r1) {
                if ($r1 != $r2) {
                    my $cost = $weapon->[0] + $armor->[0] + $rings[$r1]->[0] + $rings[$r2]->[0];
                    my $player = {
                        hp => 100,
                        damage => $weapon->[1] + $armor->[1] + $rings[$r1]->[1] + $rings[$r2]->[1],
                        armor => $weapon->[2] + $armor->[2] + $rings[$r1]->[2] + $rings[$r2]->[2]
                    };
                    my $boss_i = {
                        hp => $boss->{hp},
                        damage => $boss->{damage},
                        armor => $boss->{armor}
                    };
                    while (1) {
                        # Player deals damage
                        $boss_i->{hp} -= max($player->{damage} - $boss_i->{armor}, 1);
                        last if $boss_i->{hp} <= 0;
                        # Boss deals damage
                        $player->{hp} -= max($boss_i->{damage} - $player->{armor}, 1);
                        last if $player->{hp} <= 0;
                    }
                    if ($player->{hp} > 0) {
                        push @wins, $cost;
                    } else {
                        push @losses, $cost;
                    }
                }
            }
        }
    }
}

print min(@wins), "\n";
print max(@losses), "\n";

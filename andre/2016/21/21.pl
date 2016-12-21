#!/usr/bin/env perl

use strict;
use warnings;

use List::Permutor;

my @instructions;

sub scramble {
    my $pw = shift;
    my $len = length($pw);
    for my $inst (@instructions) {
        if ($inst =~ m/swap position (\d+) with position (\d+)/) {
            (substr($pw, $1, 1), substr($pw, $2, 1)) = (substr($pw, $2, 1), substr($pw, $1, 1));

        } elsif ($inst =~ m/swap letter (\w) with letter (\w)/) {
            eval "\$pw =~ tr/$1$2/$2$1/";

        } elsif ($inst =~ m/rotate (left|right) (\d+) steps?/) {
            my $steps = ($1 eq "left") ? $2 : $len - $2;
            $pw =~ s/(.{$steps})(.*)/$2$1/;

        } elsif ($inst =~ m/rotate based on position of letter (\w)/) {
            my $pos = index($pw, $1);
            my $moves = $len - (1 + $pos + ($pos >= 4)) % $len;
            $pw =~ s/(.{$moves})(.*)/$2$1/;

        } elsif ($inst =~ m/reverse positions (\d+) through (\d+)/) {
            substr($pw, $1, $2-$1+1) = reverse(substr($pw, $1, $2-$1+1));

        } elsif ($inst =~ m/move position (\d+) to position (\d+)/) {
            (substr($pw, $1, 1), substr($pw, $2, 0)) = ("", substr($pw, $1, 1));
        }
    }
    return $pw;
}

while (<>) {
    chomp $_;
    push @instructions, $_;
}

print "Part 1: ", scramble("abcdefgh"), "\n";

my $permutor = List::Permutor->new (split(//, "abcdefgh"));
while (my @p = $permutor->next()) {
    if ((my $pass = scramble(join("", @p))) eq "fbgdceah") {
        print "Part 2: ", join("", @p), "\n";
        last;
    }
}

#!/usr/bin/env perl

use strict;
use warnings;

use Data::Dumper;

my @instructions;
my $register = { a=>0, b=>0, c=>0, d=>0 };

while (<>) {
    chomp $_;
    push @instructions, $_;
}

sub solve {
    my $pc = 0;
    while ($pc < scalar(@instructions)) {
        my $inst = $instructions[$pc];

        if ($inst =~ m/cpy (\w+) (\w)/) {
            my $reg = $2;
            my $amount = ($1 =~ m/^(\d+)$/) ? $1 : $register->{$1};
            $register->{$reg} = $amount;
        } elsif ($inst =~ m/inc (\w)/) {
            $register->{$1}++;
        } elsif ($inst =~ m/dec (\w)/) {
            $register->{$1}--;
        } elsif ($inst =~ m/jnz (\w) (-?\d+)/) {
            my $len = $2;
            my $r = ($1 =~ m/^(\d+)$/) ? $1 : $register->{$1};
            $pc += ($len - 1) if ($r);
        }
        $pc++;
    }
}

solve();
print "Part 1: $register->{a}\n";

# Part 2
$register = { a=>0, b=>0, c=>1, d=>0 };
solve();
print "Part 2: $register->{a}\n";

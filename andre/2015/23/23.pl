#!/usr/bin/env perl

use strict;
use warnings;

use Data::Dumper;

my @instructions;
my $register = { a=>0, b=>0 };

while (<>) {
    chomp $_;
    push @instructions, $_;
}

sub solve {
    my $pc = 0;
    while ($pc < scalar(@instructions)) {
        my $inst = $instructions[$pc];

        if ($inst =~ m/hlf (\w)/) {
            $register->{$1} /= 2;
        } elsif ($inst =~ m/tpl (\w)/) {
            $register->{$1} *= 3;
        } elsif ($inst =~ m/inc (\w)/) {
            $register->{$1}++;
        } elsif ($inst =~ m/jmp ([\-\+\d]+)/) {
            my $len = int($1);
            $pc += ($len - 1);
        } elsif ($inst =~ m/jie (\w), ([\-\+\d]+)/) {
            my $len = int($2);
            my $r = $register->{$1};
            $pc += ($len - 1) if ($r % 2 == 0);
        } elsif ($inst =~ m/jio (\w), ([\-\+\d]+)/) {
            my $len = int($2);
            my $r = $register->{$1};
            $pc += ($len - 1) if ($r == 1);
        }
        $pc++;
    }
}


solve();
print "Part 1: $register->{b}\n";

$register = { a=>1, b=>0 };
solve();
print "Part 1: $register->{b}\n";

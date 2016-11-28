#!/usr/bin/env perl

use strict;
use warnings;

my %wires;

while (<>) {
    chomp $_;
    if ($_ =~ m/(\w+) (\w)SHIFT (\d+) -> (\w+)/) { # SHIFT
        my $p = $1;
        my $op = $2;
        my $shifts = $3;
        my $q = $4;

        $wires{$q}{op} = $op;
        $wires{$q}{p} = $p;
        $wires{$q}{shifts} = $shifts;
    } elsif ($_ =~ m/NOT (\w+) -> (\w+)/) { # COMPLEMENT
        my $e = $1;
        my $f = $2;

        $wires{$f}{e} = $e;
        $wires{$f}{op} = "NOT";
    } elsif ($_ =~ m/(\w+) (\w+) (\w+) -> (\w+)/) { # BITWISE AND/OR
        my $x = $1;
        my $op = $2;
        my $y = $3;
        my $z = $4;

        $wires{$z}{op} = $op;
        $wires{$z}{x} = $x;
        $wires{$z}{y} = $y;
    } elsif ($_ =~ m/(\d+) -> (\w+)/) { # SEND SIGNAL
        my $signal = $1;
        my $wire = $2;

        $wires{$wire}{signal} = int($signal);
    } elsif ($_ =~ m/(\w+) -> (\w+)/) {
        my $equal = $1;
        my $wire = $2;
        $wires{$wire}{equal} = $equal;
        $wires{$wire}{op} = "EQ";
    }
}

sub get_signal {
    my $input = $_[0];

    if ($input =~ m/^\d+$/) {
        return int($input);
    }

    # Return signal value if it exists
    if (exists($wires{$input}{signal})) {
        return $wires{$input}{signal};
    }

    if (!exists($wires{$input}{op})) {
        print "NOT INIT: ", $input, "\n";
    }
    if ($wires{$input}{op} eq "NOT") {
        $wires{$input}{signal} = ~get_signal($wires{$input}{e});
    } elsif ($wires{$input}{op} eq "AND") {
        $wires{$input}{signal} = get_signal($wires{$input}{x}) & get_signal($wires{$input}{y});
    } elsif ($wires{$input}{op} eq "OR") {
        $wires{$input}{signal} = get_signal($wires{$input}{x}) | get_signal($wires{$input}{y});
    } elsif ($wires{$input}{op} eq "R") {
        $wires{$input}{signal} = get_signal($wires{$input}{p}) >> $wires{$input}{shifts};
    } elsif ($wires{$input}{op} eq "L") {
        $wires{$input}{signal} = get_signal($wires{$input}{p}) << $wires{$input}{shifts};
    } elsif ($wires{$input}{op} eq "EQ") {
        $wires{$input}{signal} = get_signal($wires{$input}{equal});
    }

    return $wires{$input}{signal};
}

my $signal = get_signal('a');
print "$signal\n";

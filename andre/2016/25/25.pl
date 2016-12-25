#!/usr/bin/env perl

use strict;
use warnings;

use Data::Dumper;

my @lines;
my $counter = 0;
my $register = { a=>$counter, b=>0, c=>0, d=>0 };

while (<>) {
    chomp $_;
    push @lines, $_;
}

sub solve {
    my @instructions = @lines;
    my $pc = 0;

    my @output;

    while ($pc < scalar(@instructions) and scalar(@output) < 8) {
        my $inst = $instructions[$pc];

        if ($inst =~ m/cpy ([\w\-]+) ([a-z])/) {
            my $a = $1;
            my $reg = $2;
            $a = ($a =~ m/\d+/) ? $a : $register->{$a};
            $register->{$reg} = $a;
        } elsif ($inst =~ m/inc (\w)/) {
            $register->{$1}++;
        } elsif ($inst =~ m/dec (\w)/) {
            $register->{$1}--;
        } elsif ($inst =~ m/jnz (\w+) ([\w\-]+)/) {
            my $reg = $1;
            my $len = $2;
            $reg = ($reg =~ m/\d+/) ? $reg : $register->{$reg};
            $len = ($len =~ m/\d+/) ? $len : $register->{$len};
            $pc += ($len - 1) if ($reg);
        } elsif ($inst =~ m/tgl (\w)/) {
            my $val = $register->{$1};
            my $ins = "";
            my $new_ins = "";

            if ($pc + $val < scalar(@instructions)) {
                # One parameter
                if ($instructions[$pc + $val] =~ m/^(\w+) \w+$/) {
                    $ins = $1;
                    $new_ins = ($ins eq "inc") ? "dec" : "inc";

                # Two parameters
                } elsif ($instructions[$pc + $val] =~ m/^(\w+) \w+ \w+$/) {
                    $ins = $1;
                    $new_ins = ($ins eq "jnz") ? "cpy" : "jnz";
                }
                $instructions[$pc + $val] =~ s/$ins/$new_ins/;
            }
        } elsif ($inst =~ m/mul (\w) (\w) (\w)/) {
            $register->{$1} += int($register->{$2}) * int($register->{$3});
        } elsif ($inst =~ m/out (\w+)/) {
            my $val = $1;
            $val = ($val =~ m/\d+/) ? $val : $register->{$val};
            push @output, $val;
        }
        $pc++;
    }

    return join('', @output) =~ m/^(01)\1+0?$/;
}

$register = { a=>++$counter, b=>0, c=>0, d=>0 } while (not solve());
print "Part 1: $counter\n";

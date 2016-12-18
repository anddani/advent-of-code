#!/usr/bin/env perl

use strict;
use warnings;

sub solve {
    my $line = shift;
    my $rows = shift;
    my $num_safe = () = $line =~ m/0/g;

    for my $row (1..$rows-1) {
        my $new_line = "";
        for my $col (0..length($line)-1) {
            my $char;
            if ($col == 0) {
                $char = substr($line, 1, 1);
            } elsif ($col == length($line)-1) {
                $char = substr($line, length($line)-2, 1);
            } else {
                $char = int(substr($line, $col-1, 1)) ^ int(substr($line, $col+1, 1));
            }
            $new_line .= $char;
        }
        $num_safe += () = $new_line =~ m/0/g;
        $line = $new_line;
    }
    return $num_safe;
}

while (my $line = <>) {
    chomp $line;

    $line =~ s/\^/1/g;
    $line =~ s/\./0/g;

    print "Part 1: ", solve($line, 40), "\n";
    print "Part 2: ", solve($line, 400000), "\n";
}

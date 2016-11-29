#!/usr/bin/env perl

use strict;
use warnings;

while (<>) {
    chomp $_;
    my $password = $_;

    $password = valid($password);
    print "first: $password\n";

    $password++;
    $password = valid($password);
    print "second: $password\n";
}

sub valid {
    my $password = $_[0];

    until ($password !~ m/[iol]/ &&
           $password =~ m/(?:(.)\1.*){2,}/ &&
           increasing($password)) {
        $password++;
    }
    return $password;
}

sub increasing {
    my $c = $_[0];

    for my $i (0..length($c)-3) {
        my $a = ord(substr($c, $i, 1));
        my $b = ord(substr($c, $i+1, 1));
        my $c = ord(substr($c, $i+2, 1));
        if ($a == $b-1 && $b == $c-1) {
            return 1;
        }
    }
    return 0;
}

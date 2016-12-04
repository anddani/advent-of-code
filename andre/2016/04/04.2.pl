#!/usr/bin/env perl

use strict;
use warnings;

use Data::Dumper;

sub decrypt_name {
    my @crypt = split //, $_[0];
    my $id = $_[1];
    my $decrypted = "";

    for my $c (@crypt) {
        if ($c ne " ") {
            for (1..$id) {
                $c++;
            }
        }
        $decrypted .= substr($c, -1);
    }
    return $decrypted;
}

my $sum = 0;

while (<>) {
    my %frequencies;
    chomp $_;
    $_ =~ m/([a-z\-]+)(\d+)\[(\w+)\]/;
    my $name = $1;
    my $crypt = $1;
    my $id = int($2);
    my $checksum = $3;

    $crypt =~ s/-/ /g;
    $name =~ s/-//g;
    my @name = split //, $name;

    for my $char (@name) {
        if (exists($frequencies{$char})) {
            $frequencies{$char}++;
        } else {
            $frequencies{$char} = 1;
        }
    }

    my @largest = sort {
        $frequencies{$b} <=> $frequencies{$a}
                        ||
                    $a cmp $b
    } keys %frequencies;

    if (join('', @largest[0..4]) eq $checksum) {
        $sum += $id;
        if (decrypt_name($crypt, $id) =~ m/northpole/) {
            print "$id\n";
        }
    }
}

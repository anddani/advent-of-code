#!/usr/bin/env perl

use strict;
use warnings;

use Digest::MD5 qw/md5_hex/;

sub hash {
    my ($salt, $index, $stretch) = @_;

    my $hash = md5_hex($salt.$index);
    $hash = md5_hex($hash) for (1..$stretch);
    return $hash;
}

sub solve {
    my ($salt, $stretch) = @_;
    my $index = 0;
    my $num_ok = 0;
    my $hash = "";
    my $five_digits;
    my @hashes;

    push @hashes, hash($salt, $_, $stretch) for (0..999);

    while (1) {
        # Always keep 1000 hashes from current hash
        my $c = $hashes[$index%1000];
        $hashes[$index%1000] = hash($salt, $index+1000, $stretch);

        if ($c =~ m/(\w)\1\1/) {
            $five_digits = $1 x 5;
            if (grep(/$five_digits/, @hashes)) {
                $num_ok++;

                if ($num_ok == 64) {
                    return $index;
                }
            }
        }
        $index++;
    }
}

while (my $salt = <>) {
    chomp $salt;
    print "part1: ", solve($salt, 0), "\n";
    print "part2: ", solve($salt, 2016), "\n";
}

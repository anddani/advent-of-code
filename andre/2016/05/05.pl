#!/usr/bin/env perl

use strict;
use warnings;

use Digest::MD5 qw/md5_hex/;

while (<>) {
    chomp $_;
    my $id = $_;
    my $num = 0;
    my $hash = "";
    my $first = "";
    for (1..8) {
        $num++ while (($hash = md5_hex($id.$num)) !~ m/^00000.*/g);
        $first .= substr($hash, 5, 1);
        $num++;
    }
    print "first password: $first\n";

    my @second = ("", "", "", "", "", "", "", "", "");
    $num = 0;
    $hash = "";
    while (length(join("", @second)) < 8) {
        $num++ while (($hash = md5_hex($id.$num)) !~ m/^00000.*/g);

        if ($hash =~ m/^00000([0-7])(.)/) {
            $second[int($1)] = $2 if ($second[int($1)] eq "");
        }
        $num++;
    }
    print "second password: ", join("", @second), "\n";
}

#!/usr/bin/env perl

use strict;
use warnings;

use List::Util qw/min max/;

my $instructions = {};
my $vals = {};
my $output = {};
my $result1 = 0;
my $result2 = 1;

sub solve {
    for my $id (keys %{$vals}) {
        my $bot = $vals->{$id};

        if (scalar(@{$bot}) == 2) {
            my $low = min(@{$bot});
            my $high = max(@{$bot});
            $result1 = $id if ($low == 17 and $high == 61);

            my $bot_i = $instructions->{$id};

            push @{$vals->{$bot_i->{l}[1]}}, $low if ($bot_i->{l}[0] eq 'bot');
            push @{$output->{$bot_i->{l}[1]}}, $low if ($bot_i->{l}[0] eq 'output');

            push @{$vals->{$bot_i->{h}[1]}}, $high if ($bot_i->{h}[0] eq 'bot');
            push @{$output->{$bot_i->{h}[1]}}, $high if ($bot_i->{h}[0] eq 'output');

            $vals->{$id} = [];
            if ($output->{0} and $output->{1} and $output->{2}) {
                $result2 = 1;
                $result2 *= $output->{$_}[0] for (0..2);
            }
        }
    }
}

while (<>) {
    chomp $_;
    if ($_ =~ m/(\d+) .* (\w+) (\d+) .* (\w+) (\d+)/) {
        $instructions->{$1} = {
            l=>[$2, $3],
            h=>[$4, $5]
        };
    } elsif ($_ =~ m/(\d+) .* (\d+)/) {
        $vals->{$2} = [] unless defined($vals->{$2});
        push @{$vals->{$2}}, $1;
    }
}

while ($result2 == 1 or $result1 == 0) {
    solve();
}

print "$result1\n";
print "$result2\n";

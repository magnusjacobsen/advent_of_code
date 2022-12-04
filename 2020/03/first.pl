my $x = 3;
my $y = 1;
my $trees = 0;

my @array;
open(my $input, '<', 'input') or die $!;

while (<$input>) {
    chomp;
    push @array, [ split '' ];
}

while ($y < scalar @array) {
    my @linearray = @{@array[$y]};
    if (@linearray[$x % scalar @linearray] eq "#") {
        $trees += 1;
    }
    $x += 3;
    $y += 1;
}

print "trees: $trees\n";
my @array;
open(my $input, '<', 'input') or die $!;

while (my $line = <$input>) {
    chomp $line;
    push @array, [ split '', $line ];
}

my @rights = (1,3,5,7,1);
my @downs = (1,1,1,1,2);
my @trees = (0,0,0,0,0);
my $score = 1;

foreach $i (0..4) {
    my $x = @rights[$i];
    my $y = @downs[$i];
    while ($y < scalar @array) {
        my @linearray = @{@array[$y]};
        if (@linearray[$x % scalar @linearray] eq "#") {
            $trees[$i] += 1;
        }
        $x += @rights[$i];
        $y += @downs[$i];
    }
    $score *= $trees[$i];
} 

print "trees: @trees, score: $score\n";
# Not my best work

This is a poor implementation of some sort of Dijkstra's algorithm.
I basically iterate all of the edge nodes and see which is the next
lowest node. Rinse and repeat.

I used a HashMap to store nodes and that is not efficient. At this point
I couldn't be arsed to do better so, when the input was larger I added
a small optimisation, and waited. For 45 minutes on this fairly puny 
machine.

Also, again from boredom and impatience, I used octave to generate the
expanded input for the second part of the problem.

Who knows? When I retire and have more time I may revisit this and give
it the attention it deserves!

I won't.



    [336]% pwd
    /home/simont/aoc2021/day15
    [337]% bash second.sh input > out
    [338]% cd first/
    [339]% time cargo run --release < ../out

    ...

    best: Some((495, 499)) Some(2968)
    best: Some((499, 498)) Some(2969)
    best: Some((498, 497)) Some(2970)
    best: Some((497, 499)) Some(2971)
    best: Some((499, 496)) Some(2972)
    best: Some((499, 499)) Some(2976)

    real    44m36.796s
    user    44m27.918s
    sys     0m22.959s

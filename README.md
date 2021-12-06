# advent21
[Advent of Code 2021](https://adventofcode.com/2021)

Warning! Possible spoilers for advent of code solutions below.
Mostly rust solutions for the excellent puzzle-athon.

### Day 4 

This was a fun one, and leans heavily on how to represent the problem
in a sane way.
As usual, do the legwork to keep things tidy, and part 2 is trivial.
Personally, I spent most of the time writing the parser, and tracking 
down a counter that I'd put in at the wrong loop level.

Otherwise, keep your head and you'll be fine. Things to think about:

* How do I know when a row is full? What do I need to track?
* Same with columns?
* And what do I need to know to calculate the score?

Work out how to update this with each call and you're fine. There aren't that
many boards, so worth spending extra memory if it keeps things simpler.

### Day 5

Keep calm and read the question carefully for this one. As usual the problem
description can suggest inefficient ways to model.

* You only need to worry about discrete points on the ocean floor. (This is
particularly important for part 2).
* Diagonals will *always* be at 45 degrees. This simplifies things a lot.
* You probably don't need to map the whole ocean floor - unless you want a 
visualization? Having said that the whole map isn't prohibitively large.

Probably the biggest thing to note though is that you want to work on the 
ocean map, rather than looking for crossings between lines. Having said that
it depends on the data. In this case the lines are relatively 
short, in terms of the points on the line. In a 
different situation you might have very long lines (mm grid over km lengths?)
and then looking for crossings would be more efficient. Always good to 
do a quick run over data and think about where the big numbers will come
from.

### Day 6

This one follows the classic idea of 'model it properly and part 2 is trivial'.
Like many of this sort of problem, the naive approach suggested in the problem
description _maybe_ doesn't scale well.

An observational hint - these lantern fish seem to live quite a solitary life.

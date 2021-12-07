# advent21
[Advent of Code 2021](https://adventofcode.com/2021)

Warning! Possible spoilers for advent of code solutions below.
Mostly rust solutions for the excellent puzzle-athon.

### Day 1

Nice gentle warm up. Covers the basics of reading input,
iterating over data.

### Day 2

Keep your head straight, follow the maths, and you'll be fine.
As usual think about:

* What 'state' do you need to keep track of?
* How do you update that state?

### Day 3

This one was interesting - it felt quite novel. 
The part 2 was a nice extension as well, adding
more to think about.

A few interesting things to think about:

* Parsing 'binary' data. Depends on your language so
always fun to play with.
* How to identify 'most' and 'least' common. Lots of 
scope for falling the wrong side of the boundaries. 
Lots of scope for off-by-one errors.
* How to do the problem efficiently? If you want to
avoid running through your data multiple times,
what do you need to keep track of?

### Day 4 

This was a fun one, and leans heavily on how to represent the problem
in a sane way.
As usual, do the legwork to keep things tidy, and part 2 is trivial.
Personally, I spent most of the time writing the parser, and tracking 
down a counter that I'd put in at the wrong loop level.

I'm not surprised that this day's stats shows a drop in completion - it's
definitely meatier than the day before. We seem to be following the 'slightly
harder bump at the weekends' tradition, which makes sense.

Otherwise, keep your head and you'll be fine. Things to think about:

* How do I know when a row is full? What do I need to track?
* Same with columns?
* And what do I need to know to calculate the score?

Work out how to update this with each call and you're fine. There aren't that
many boards, so worth spending extra memory if it keeps things simpler.

### Day 5

Keep calm and read the question carefully for this one. As usual the problem
description can suggest inefficient ways to model.

I've avoided so far wrting parsers. (And the problems 
seem to need it a lot less than last year.) I took the
opportunity to implement the `FromStr` trait for my data 
types, keeping the parsing quite neat. (Thank you Rust!)


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

### Day 7

This was an interesting one, and probably a bit more maths-y. First part is 
frustratingly similar to the normal _least squares_ regression, where the 
mean value minimizes the square distances.

Of course the minimizing the square is not the same as minimizing the absolute
distance so a few more thoughts are needed. The key question is:

* If I have a target value, how does it change if I increase it?
* And what if I decrease it?

From there it's fairly short to convince yourself that you go down
by the points you're moving towards, and up by the ones you're moving
away from. And from that, you can deduce that the optimizing value is the
median - although you still have to calculate that, and the minimized distances.

Part two then flows naturally - how does the cost change when I 
move? This then ends up being a bit more involved, but still managable
by thinking about how things change at each step.

It's interesting to think about what the traps being avoided are - for one
you want to avoid recalculating the distance from n points every step (~ n^2).

On the other hand I *did* do exactly that for tracking down a bug. The actual 
updates to the costs is a bit fiddly! Anyway, all done and crabs away!
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

### Day 8

The first part is simple, but the second part is significantly more difficult. 
The first part does give a clue on how to handle the second part.

There are multiple approaches, but it's one of those situations where a completely
generic solution is complex. (Even working out how to describe the generic problem
in code is complex!) So, many will fall back to a hand rolled solution based on 
a few ideas:

* Part 1 uniquely identifies some values as starters.
* How can you identify the rest?
* What are the intersections of digits like? Are some contained in others?
* Are some the unions of other digits?
* We don't have to just work with digits - maybe it's useful to find individual segments?
* Are there any statistical properties to take advantage of? (e.g. how often a segment appears
across all digits.)

The only major alternative is the 'scan all possibilities and check if this produces valid
digits' approach. This has to work (otherwise there's no unique decoding), but it is a lot
more expensive. The number of segment permutation is finite, and not 
insanely large so it is one of those days where a brute force approach will work.
On the other hand, the code to generate the permutations and check validity is complicated
enough that I'd hesitate to call the brute force approach the *easy* approach in this case!

### Day 9

I wondered if the calendar was starting to look a bit volcanic!
Classic iteration over neighbours type question - I should probably think about
making a library around this. (Or find an existing one!) 
The problem itself is fairly OK if you've seen this sort of thing - explore the
space, don't double count. Today the boundary conditions are fairly simple (extend
up to '9's) - I think I remember previous years with similar questions but less
obvious boundaries. There's still plenty of scope for complication.

As usual my code for this is a bit ad-hoc, hard coding the four neighbours for
each cell. It seems enough for today, but we might want something cleaner in 
future if we hit more complex problems. (Memories of last year where 2D game of
life quick evolved to n-dimensions, or extended neighbour patterns.)

### Day 10

This was probably the quickest that I've been in a while,
probably due to having to think about parsing for
the puzzles last year.

Then, the thing that I learnt was shunting train algorithms,
however you don't need anything that complicated for today.

Key observation - stacks are your friend here. It's significantly
simplified by the fact that you don't actually need to **do** anything
with the input, just match the brackets.

As always, think about what the options are at each step, and what
information you need to know to make that choice:

* I can either close the last opened bracket
* Or open a new bracket.
* I therefore need to know the last opened brakcet.
* When I successfully close a bracket, I now have the *previous*
unclosed bracket.

So a stack of opened brackets ftw.

The second half is almost disconnected (the first half 
lets you prune out those which can be ignored).
Observations to avoid being wasteful - you don't really
need to work out the completion. (But may help with debugging
if necessary.)

Finally, factor it pleasantly to avoid looping over the input
unnecessary extra times.

### Day 11

Flashing octopuses! This one is ripe for visualizations, I'll
look to head back to this if I have time.

The problem itself is a fairly fine grid update problem. The key
with this sort of thing, where you have cascading updates, is to
ensure that you update everything exactly as much as it needs to 
be updated - no more, no less. Stacks are always useful to maintaining
work lists. Maybe you want maps for avoiding double counting - or not
if you can ensure you don't double count in other ways.

In this specific problem, the issue comes down to:

* Update all the octopuses. Note which ones flash.
* Work through the flashes, updating neighbours.
* Make sure you flash octopuses which are pushed over by neighbouring flashes!
* Don't flash octopuses twice!

Once done, the two parts are fairly similar. If you've organized
the code neatly, part two is just keep running until you hit 100
flashes.

Things to make the code nicer? Try to avoid hard coding all the 
neighbours - a double loop over (-1, 0, 1) should cover you here.
Then there's a lot of twiddling if you want to squeeze performance
and/or code beauty.

Unrelated, I tried running my code on a windows box, and then 
realized that (reasonably) the library `termion` which I'd been
using for pretty console output wouldn't be windows compatible. This
pushed me to rewrite the code as a library (doing the work) and a 
small *nix specific executable for pretty output. This helpfully
makes a wasm version much easier as well. (I can now incorporate my
library into a wasm targeted project.)

### Day 12

A nice graph traversal problem today. The repeatable/non-repeatable
caves makes a nice variation, but keep calm and all will fall out nicely.

The second part - if you've structured things nicely this will be fine.
My work consisted of changing a 'seen' map from bools to ints, and re-reading
the question more carefully to understand that only *one* small cave can 
be visited twice.

Interestingly this was the first day that the borrow checker caused me problems,
largely due to extensive use of strings as keys. As a result lifetimes and
borrowing became issue, and I ended up with fairly poor performance due to
quite a lot of string copying. I wonder if I can squeeze better performance
by using integer indices throughout (and keeping a lookup function where
necessary).

_Update_:: As expected, switching to int indices as soon as possible cut timing
to about a third.

_Update_:: Also avoid cloning hash maps - surprisingly this provided very little
performance improvement. (But it is nicer code.)

_Update_:: Squeezed out a few more ms (getting under 100ms on my maching) by
avoiding hash maps wherever possible now we're using integer indices.

### Day 13

I loved today! Plot some dots on paper, fold the paper, read the paper.

I don't think there's too much complication. The parsing is relatively simple.
You need a little bit of geometrical intuition in order to work out the
new coordinates after folding.

There are definitely some choices on how to model the dots - some things to 
make like easier are to note that you don't need to _merge_ dots really, you
just need to note whether a location is marked or not. Folds compose nicely, 
and also worth noting that the number of folds is pretty small - i.e. you're
probably fine to try all folds on all points.

But largely it's a question of not making this more complicated than it 
needs to be. And having fun with visualizations if you like.

It reminds me of a 2019 (?2018?) puzzle involving moving stars. That one
was simulate the stars movement, read a message when they align.

_Update_ : I looked it up. It's 2018, Day 10 with the stars spelling messages.


### Day 14

A very classic idea here - naive implementation completes fine for 10 iterations,
but explodes to unworkable levels for 40 iterations.

Then a smart algorithm completes in a few milliseconds.

There are a few features that make this more interesting/fiddly:

* You're looking at pairs.
* You're looking for most/least common values.

Eventually though, this boils down to memoization. As always a little
extra can be squeezed out by getting away from chars/strings and to 
integers as soon as possible. It's incredibly interesting how these
hugely complex structures resolve quite quickly once the recursive 
nature is leveraged.

### Day 15

It wouldn't be advent of code without some path finding going on!
This one is a fairly straightforward implementation - when that's the
case I find it quite useful to do the implementation without 
looking things up - partly relying on memory, partly relying on 
re-deriving the ideas.

I see some people complaining on reddit about this being a copy
paste type problem - but that's really the case with any AoC problem.
If you really care and just want the optimal solution you can 
do a google search and pull down a solution in your language of choice.

As always with this sort of thing, the interesting bit is the 
detailed implementation, particularly if you're pursuing performance.

Plus, there's also a lesson in reading the question properly to
make sure you get the right repetition behaviour.

_Update_ : In a lesson for using the right data structure, using
a min heap implementation works in about a fifth of the time of
using a (continually) resorted list.
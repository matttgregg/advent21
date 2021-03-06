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

_Update_ : This seemed to lend itself to visualizations, however just
plotting the best path cost to each point leads to a rather 
simple image. However, I noticed that it's *mostly* equal to 
a simple distance function from the top left corner. If you 
strip away that function (scaling so that it matches at the start 
and end points), you get a lot more interesting detail. It's
also strangely organic, reminiscent of clouds.

### Day 16

I like the way that AoC shifts between deceptively simple but 
computationally hard problems to failry complicated problems
which are fairly cheap in terms of computation. This definitely 
falls into the latter category. There are a lot of details
to encode, but as long as you keep them straight you don't need
to worry about performance problems.

As I've been working through the excellent [Crafting Interpreters](https://craftinginterpreters.com/)
I decided to take a fairly standard, hand written approach.

My program looks very much like a tokenizer (where all my tokens
are `1` or `0`), and I build up a tree structure.
The second part, evaluating the message, was happily trivial
given I'd passed the data properly for my first part.

I did take a slight shortcut on summing the version values - I
kept a total as I was reading them, but it would be easy enough
to evaluate from a tree walk as well.

### Day 17

Ho, ho, ho! This one was a fun day. It was particularly nice in 
that there are (at least) two approaches:

* Simulate the trajectories! Then try possibilities to find which cross.
* Don't simulate that trajectories but use the math.

I went for the second, but the first is perfectly workable. In
a real life setting (and a maybe more complex system) I can easily
see using the first to get a handle on the problem, and then trying
the second when worrying about performance.

The maths isn't too hard - A-Level (pre-degree) should give you 
everything you need to derive from first principles. It's 
actually a lot of fun but you do need to take care. I wasted
too much time on:

* An inverted sign.
* Forgetting that the x-velocity couldn't go in reverse.

Admittedly, it's the sort of thing that if I'd been simulating
full trajectories the x-reverse problem would have been 
obvious sooner!

Very nice variation again, from the previous days or high 
efficiency (15th) and extra computer science-y (16th).

### Day 18

I woke up this morning and though _this looks fun_.

Nested tree parsing, interesting production rules - again
extremely thankful that I've been reading up on compiler
design and parsing this year, and was able to implement
pretty much as planned:

* Use a stateful machine to walk through tokens left to right.
* Have useful consume/peek/move left/right functions.
* Have useful (try) consume/peek number (/pair) helper functions.

The best advice for myself - do it by small pieces, check each 
small piece, build everything together. Which meant that when I
did make mistakes I was debugging very specific chunks of 
functionality. Roughly in order:

* Handle explosions. (Recognise explosions -> fix right -> fix left).
* Handle splits. (Much simpler than explosions.)
* Handle complete reduction.
* Handle sums.
* Handle magnitudes. (I didn't even think of this until I'd got
the sums working properly.)

The sort of problems I hit were:

* Off by one errors when working back to the 'left' number.
* Not trying explodes again after every *single* split.
* Look ahead for consuming '[x,y]' and restoring the state
if not found. (This was easy for the explode portion as the
problem was set up so that the depth counter unambiguously 
signalled when you needed to read a pair, so no need to peek.
I found more complications when trying to calculate 
magnitudes in this way.)

However, there weren't too many surprises, and the examples
given were rich enough to find all my problems. Once the
example input passed, my real data ran without problem.

The second part I just did an exhaustive pair check. I didn't
obviously see any quick shortcuts given the non-trivial rules.
As a result the second part was a check that the first part was
_efficient enough_. As it stands, both parts complete for me
in about a second. There are almost definitely some improvements
but I don't think anything many orders of magnitude.

### Day 19

Woke up early, scanned through through problem and smiled.
Given a number of overlapping 3D scans, work out how they fit together.

That's the hard bit. Count the points and separation once you've
re-assembled the scans isn't too bad.

Flashbacks to the last years sea monster map, which I wasted far
too much time on. Key differences are:

* This is in 3D!
* Alignment is on points, not edges.
* What you do with the points when you've found them is
significantly easier, I'm thinking to compensate for the
more complicated data.

Happily, having learnt my lessons, I went through this more 
carefully and got through the problem much more quickly and
without any major problems. One thing I did do different was
put in sanity checks that when I performed alignment for a single
point, that point really did align with the transform I worked out.

Key observations which made life easier:
* You don't need to align all axes at once. Just start with one
axis.
* Again work in stages. Check you can align a single point, then 
a scanner with an x direction, then a scanner with all directions.
* Don't duplicate your point transformation logic. I think this is
what broke me last year. As long as you keep transforming
consistently, it doesn't matter exactly how you track your
transforms.

To elaborate on the last point, the things that can get confused
are like: Did I offset the flipped value? Did I offset then rotate?

For part 2, I rushed through and calculated beacon distances rather 
than scanner distances, but even with that number, an O(n^2) algorithm
isn't that costly. Quick shortcut for finding a scanners position - 
transform the origin point.

I'd be willing to guess that this will be the toughest problem this
year, although happy to be proved wrong. Looking at the stats page
people are definitely feeling the difficulty ramping up.

### Day 20

As suspected a bit of a relaxing one after yesterday. Mostly it's 
a game of life variation - a grid of data with some update rules.
There's one particular gotcha to watch out for. Although I didn't 
account for it initially, I _suspected_ it might come up and 
visualizations confirmed it. As I result this wasn't frustrating.

### Day 21

Another fun one. The first part is a fairly straightforward _play
this game in code_ . (Straightforward doesn't mean I don't get 
caught up in silly off-by-one errors in the modulo arithmetic. As
always copious checks and debugging statements are your friends.)

As is often the case for these games there are a reasonable number
of hard coded constants. I could parameterize *but* without knowing
what's going to be useful for the second part, this can be wasted 
effore. (e.g. I did parameterize die size, but didn't really get 
any use from it.)

In the end I didn't re-use any code from the first part in my second
part.

For the second part - just a look at the size of the results should
make it clear that you're not going to be simulating every single 
game. So, we fall into:

* How to paramaterise the 'play a round' step sanely.
* What state to track to allow us to cache results.
* Any other observations to squeeze out performance.

As usual I've gone for aggressive caching while avoiding
too much complication. I did start fiddling with my own hashing
and a straight array to avoid hash lookups, but mistakes
meant that I was hitting an unhelpful rust error when 
allocating my large empty array. I may revisit it, but can't
imagine it will provide orders of magnitude performance versus
the current code.


_Update_ : Did go back and re-implement as a vector with a 
hand written hash, and approx. 10x improvement. Down to 
around 2ms on my laptop.

_Update_ : After browsing reddit, I saw a comment that you
can reduce the memory cost due to the results being symmetric.
(i.e. The wins for player 1, player 2 are the same as those for
player 2, player 1 with the scores reversed.) I tried this, it's
a relatively minor bit of code to implement - it does save memory but
runtime isn't particularly affected.

_Update_ : Also noticed some interesting things about the stack
allocation limits varying between systems. Although the array 
version worked fine for mac, it failed both on windows and the
linux versions run by github integrations. Switching to vectors
to put allocation onto the heap cleared up this issue. (Interestingly
simply boxing the array still caused problems for tests - I still need
to work out why.)

### Day 22

This was an immensely satisfying day!

The problem was well stated and clear, but how to do this efficiently
was not immediately clear. Anyway, I started with a naive 
implementation just to unlock part two - and unsurprisingly part
two was to extend the analysis to the full spatial region.

I had no idea how to do this. I don't have a background in
computer graphics, but suspected this is something that would
be done commonly in that case, simply looking at overlapping 
rectangles. I avoided googling for 'overlapping rectangles' and instead
had a coffee. These were the thoughts I had:

* There are far too many points to track them all.
* Try and think in 2D space.
* If two rectangles overlap, the overlapping region is another rectangle.

This got me thinking that I could always break two overlapping rectangles into
multiple non-overlapping rectangles. e.g. If you take a rectangle out of the 
middle of a rectangle, you'll get nine smaller disjoint rectangles.

I briefly started coding on this and thought:
* I'm going to get a lot of rectangles quickly.
* And I haven't done anything smart for checking intersections - so it's 
going to be roughly an n^2 cost.
* The geometry for working out the sub rectangles is going to be a pain.

So, I thought again on what I *really* wanted to know. I *really* wanted
to just know how many cubes were lit, not exactly where they were. So for
the overlapping rectangles, I just want to know the contributions of the various parts.

This got me going on the right track:

* I only need to know contributions.
* For *on* cuboids, contributions are +1 for themselves, and -1 for overlaps with other lit cubes.
* For *off* cubes there's a contribution of -1 for overlaps with other lit cubes, but no contribution
for themselves on empty space, because they don't affect later cuboids.

This meant that:
* My overlap checks would still be (no. of cubes)^2 - but I wouldn't be creating quite so 
many cubes. In particular, for 'off' cubes I'd only collect the intersections.

The last tricky bit was working out the intersections with 'contribution -1' pieces - i.e.
these patches which were compensating for overlapping 'on' segments, or active 'off' segments.
A little bit of thinking, and debugging the tiniest example got me that last star.

All in all, despite the n^2 in terms of cube count, this runs for me in ~60ms. Even within 
this algorithm, there are almost definitely smart ways to squeeze out more performance. For example
I wonder about clever ordering to make the intersection checks faster - but it's not 
critical in the context of AoC. 

I'll be interested to see the reddit take on this one. I see the completion rate is
fairly low again, which doesn't surprise me.

### Day 23

This was an interesting one!

For one thing it was at least as difficult as any other this year. There's quite a lot
to keep track of, even once you recognise it as a disguised path finding algorithm.

* The rules for which locations are accessible are non-trivial.
* The rules for costs are non-trivial.
* An A* like cost is non-trivial to work out, if you want to optimize that way.
* Handling a general form for both parts is non-trivial and harder to optimize.

Nevertheless, it's an interesting puzzle with a lot to work on. Definitely running at
my slowest so far at ~3s, but this late in the day it's hard to build up the enthusiasm
to further optimize right now.

Final interesting thing to note, is that a lot of people found it easier to solve by
hand! Despite that, there's a lot of satisfaction in writing out the generalized solution. 

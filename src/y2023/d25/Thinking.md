There is a graph given. Find three connection that when removed will split the graph into two graphs

I have to start somewhere at random and discover the graph, find a "weak spot"

bruteforce: Eg. What happen when removing this three links?

The abstract problem I basically min-cut solution. I can employ Stoer-Wagner algorthm:  http://dl.acm.org/doi/epdf/10.1145/263867.263872;

This algorytm published in 1995 proposes solution for finding a min cut between fixed source and any sink node. Which is idea for this problem. Before that there were flow algorythm using fixed s,t duality.

So the algorythm consist of following steps:

let G = input graph

while len(G) > 1:

    ----------1
    find "natural" cut by walking from random node in the direction of next tightly connected node   
      -1- B 
    /     |
    A     1
    \     |
      -1- C
    in this case go to B (or C). Then merge nodes and edges and sum weights
    
    AB - 2 - C

    Do that repeatiedly till two nodes are remaining. now the "natural" cut is found.

    2 ----------------
    Go to reference graph and merge last two nodes. (compare cut value with min cut value)


#####
Update: Implemented this steps but there are things unclear now.

I need to find 3 edges so that when deleted seperate graph into two groups. I planned to do this using the Stoer-Wagner algorythm.

I think I need to revisit materials regarding algorythm. It might be as simple as iterating till I find a cut of 3 but I would like to have better understanding of theory.
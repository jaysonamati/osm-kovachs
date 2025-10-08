# Development notes

## Osm data Counts (In parallel)

Finished counting in: 91.965634127s
Nodes: 54328084
Ways: 7684884
Relations: 7734

### Comments

This takes a minute, the host machine specs are as follows:

- OS: Ubuntu 22.04.4 LTS x86_64
- 5.15.0-117-generic
- CPU: Intel i5-6300U (4) @ 3.000GHz
- GPU: Intel Skylake GT2 [HD Graphics  

## After filtering with key="highway" (sequential) while creating TransportSpace (Medium) objects

This takes significantly longer; should find a way to optimize

Finished counting in: 457.688514105s
ways:  670903
nodes: 12528560
Created mediums: 670903

## Things to note and think about

- Check out quadTrees for data processing and making associations between data
- There is an issue with getting nodes in a way, and we have to figure a way around this, or maybe the data is just not there.
- Processing the giant data to the json format we need takes a lot of time, think of optimizations to curb the excessive computations.
- For instance consider creating the mediums with location while doing the `par_map_reduce`
- We should split the task of populating medium positions into two:
  - First we create the mediums with only node refs
  - We par_map_reduce again with only the nodes and then add the relevant nodes to the medium with the same refs/

## Questions to answer

- How fast is it to get node information given a node id?
- So we don't have to populate mediums with lats & long

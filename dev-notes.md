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

# BFS_MoreLanguages
test project for compare bfs speed on difference languages 

Any tests: 100000 runs. Area 100x100. Walls generate used same algorith.
On one run: path founds from (1,1) to (98,98)

## Mac Mini 2018 3,2 GHz 6-Core Intel Core i7

### Kotlin (JVM, openjdk-15) (outdated)
Avg. Time: 14500 ms

### Swift usage Unsafe API, (Release, -O, SWIFT_DISABLE_SAFETY_CHECKS = YES)
Avg. Time: 2600 ms

### CPP (Release, -03)
Avg. Time: 2500 ms

### Rust (Release, --all-features)
Avg. Time: 3250 ms

## MacOS 10.15.3 3,6 GHz 8-Core i9700k @Karloid

### Kotlin (JVM, openjdk-11) Jar
Avg. Time: 3818 ms
Cpu time: 4000 ms 

### Kotlin (GraalVM 20.3.0)
Avg. Time: 4724 ms
Cpu time:  4670 ms

### CPP (Release, -03)
Avg. Time: 2318 ms 

## You PC? :)

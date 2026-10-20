# Memseries

memseries is a command line tool for generating a timeseries of program RSS usage.

The program being analyzed is run `n` times for a certain duration, with the memory usage progression across multiple runs being used to calculate the mean and standard deviation of the timeseries. The mean series is of course a good summary of the progression, to the degree that the stdev series is reasonably low. Points of high variance may be of particular interest.

Each process is killed at the end of the specified duration.

The stdout of the child processes is suppressed.

Run `cargo run --release -- --help` to see the command line interface.

For example, this will run the `sample` program with default settings and store the output in `sample.memseries`:

`memseries /usr/bin/sample > sample.tsv`

`cat sample.tsv`:

````
mean	stdev	0	1	2	3	4
341.2	410.63044212527643	858	7	5	830	6
17927.6	7244.128977316738	12778	14009	27767	9687	25397
28452.8	1736.7193670826862	30153	25109	28934	28808	29260
27752.4	4106.892333626469	29778	29111	29197	31023	19653
28772	5128.849110668011	32346	18569	30938	30999	31008
37028.4	4875.324547145536	38916	28627	34695	41693	41211
56039.2	17363.09464813229	64864	32941	37320	71724	73347
63330	13168.42880529032	69561	49024	45992	75906	76167
70507	8766.174513435173	72641	71709	53594	77214	77377
74630.2	4090.559345615246	73365	75587	67395	78818	77986
75933.4	2092.3770788268907	74979	76648	72391	77205	78444
```

For this program, the runs diverge in the medium term, but only to arrive at a fairly similar place by the end.

In the mean, it can be seen that RSS usage does indeed grow over time, though in somewhat bursty fashion. 

Load this into your favorite spreadsheet or any other analysis tool to generate charts, etc.

## License

memseries is licensed under the GPL version 3.

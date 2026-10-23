# Memseries

memseries is a command line tool for generating a timeseries of program RSS usage.

The program being analyzed is run `n` times for a certain duration, with the memory usage progression across multiple runs being used to calculate the mean and standard deviation of the timeseries. The mean series is of course a good summary of the progression, to the degree that the stdev series is reasonably low. Points of high variance may be of particular interest.

Each process is killed at the end of the specified duration.

The stdout of the child processes is suppressed.

Run `memseries --help` to see the command line interface:

```
Generates timeseries of program RSS usage

Usage: memseries [OPTIONS] <CMD>

Arguments:
  <CMD>  The program whose RSS to track

Options:
  -n <N>                            Number of times to run the program we're analyzing [default: 5]
  -d, --duration <DURATION>         How long each run lasts; as parsed by duration_string::DurationString [default: 2h]
  -s, --sample <SAMPLING_INTERVAL>  Sampling interval; how often RSS is measured and recorded; as parsed by duration_string::DurationString [default: 1m]
  -b, --burn <BURN_IN>              How many runs to perform as burn-in before tracking [default: 1]
  -u, --units <UNITS>               Units to report results in; as parsed by byte_unit::Unit::parse_str [default: KB]
  -p, --precision <PRECISION>       Precision of output, as understood by Rust (number of digits after decimal place) [default: 0]
  -h, --help                        Print help
  -V, --version                     Print version
```

For example, this will run the `sample` program with megabyte units, a single decimal point of precision, and running 3 processes for 4 hours each; the default burn-in of a single run is overridden to have 0 burn-in runs. The  will  default settings and store the output in `sample.memseries`:

`memseries -u MB -p 1 -n 3 -d 4h -b 0 /usr/bin/sample > sample.tsv`

`cat sample.tsv`:

```tsv
mean	stdev	0	1	2
0.0	0.0	0.0	0.0	0.0
331.5	1.2	331.9	332.6	329.9
335.0	1.5	336.5	335.6	332.9
339.2	3.0	343.2	338.5	335.8
341.9	2.8	345.5	341.4	338.8
345.3	3.3	349.7	344.4	341.7
348.0	3.0	351.9	347.3	344.7
350.6	2.6	354.0	350.2	347.6
353.4	2.4	356.5	353.2	350.6
356.6	2.8	360.3	356.1	353.5
```

For this program, the runs are quite similar (as revealed by low standard deviation), with all increasing memory usage over the run, as summarized by the mean.

Load this into your favorite spreadsheet or any other analysis tool to generate charts, etc.

## License

memseries is licensed under the GPL version 3.

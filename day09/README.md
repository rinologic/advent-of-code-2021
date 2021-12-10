# DAY 09

Starting today I will use a new method of loading the puzzle data file into the program.  Please take a look at the Makefile if you are interested.  The data will be coming in through the command line by using the command cat and piping the output to the Rust program.  There is a single method in the script, load_data() that will grab it.  To run the program against the sample data, open a command prompt and type:
```bash
make sample
```
To run it against the standard data type:
```bash
make
```
This assumes the sample data is called data_sample.txt and the standard data is called data.txt though those values can be changed in the Makefile.

To create a new day's project, I simply run the following commands from the parent directory:
```bash
cargo new dayX
cp ../dayX-1/Makefile dayX
```
Then update the first line of the Makefil with that day's day number:
```bash
day = dayX
```
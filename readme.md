# Cron parser
A simple script that takes a cron line of five time fields (minute,hour,day of month,month,day of week) and a command as a single argument (wrapped in ""), and parses the times it will run. Handles fields containing 0-9,-,*,/ and comma separated ranges.
The output will be a table containing a list of times per field along the command.
The input is given via argument runtime, while the output is given via cout in the cmd window. (If required, the > command can be used to create a text output)

### Running
The application can be run via the "target" folder after building it with cargo (cargo build). It can also be run directly via cargo run command.

### Constraints
* The application will not accept anything other than the inputs specified above, i.e. other characters that cannot be parsed to numbers.
* The application will not accept an input outside the range of the field (i.e. 1-13 in the "month" field)
* The application will not run unless given an argument at runtime

### Other

* Along the application are some of the unit tests I used during development. 

Have you ever used the sleep command, only to be left wishing you could actually see the time counting down in real time? Well, now you can!

# Usage

>`sleepview [OPTIONS] [SWITCH] DURATION ...` or `sleepview [OPTIONS] DURATION[SUFFIX]...`

>DURATION: the amount of time to count down in seconds. Can be specified in combination with switches, or omitted entirely with switches present. Using a timestamp disables other duration switches, and only one of each other switch is allowed. Multiple non-timestamp durations will be added together.

>SUFFIX: can be 's', 'm', 'h', or 'd' for seconds, minutes, hours or days. Multiple durations of any kind will be added together. This is considered a fallback method, but still works with duration switches present.

>SWITCHES:

>-h :	Show this help message and exit.

>-d :	Specify days.

>-H :	Specify hours.

>-m :	Specify minutes.

>-t :   Specify a timestamp, in the form (D)D:(H)H:(M)M:(S)S(.DEC) -- days, hours, minutes, seconds, decimal portion.

>OPTIONS:
>Note: use short options only

>-f :\t(full, boolean) Show full width of timestamp, regardless of target time. Without this option, fields in the display format that will always show zero will be omitted.

>-n :\t(no_newline, boolean) Do not append a new line when the program finishes naturally -- this generally causes the output to be overwritten by either the prompt or any other output on the same line as the countdown output.

>-j :\t(json, boolean) Output data as json. Not recommended for normal use. Compatible with -f option.

>-o :\t(output, string) Specify an output file path, either absolute or relative to the current working directory. Can also be `-` for standard output, which is the default when no output path is specified.

>-u :\t(up, boolean) Count up from 0 to the target time, instead down from the target time to 0.

>-r :\t(rate, numeric) The minimum amount of time to wait as calculated by ( 1 / RATE ) seconds, if the amount of time left to count is greater than the calculated value. Otherwise, the amount of time between loops is cut to maintain accuracy to the specified target time.

>-R :\t(resolution, character/integer) Specify the resolution of the output format. Applies to both json and default formats. Can be one of \"m\" or 0 (milliseconds), \"s\"/\"S\" or 1 (seconds), \"M\" or 2 (minutes), \"h\"/\"H\" or 3 (hours), OR \"d\"/\"D\" or 4 (days). This omits the number of fields specified, and so should not attempt to omit more fields than would be displayed without the -f flag.
";

# Installation

`cargo install sleepview`
(Requires rust, of course.)
There are also two mutually-exclusive features that can be provided: `mold` and `gold`. Just add the `--features=...` option set to one of them to use the respective linker plugin.
Note that mold requires both `clang` and `mold` to be installed and in the PATH, while `gold` requires `gold` to be in the PATH.

## Important usage details
 - `crossterm` dependency SHOULD ensure cross-platform reliability, but there might be exceptions. If a case is encountered where it does not work as expected (probably due to unexpected escape code handling), feel free to send feedback including:
    1. What happened
    2. Important details about your terminal environment
    3. The CPU architecture and OS of your machine

    (It would probably be good to report any display issues to the crossterm team as well.)

 - This WILL use more cpu resources than the standard `sleep` command, because it has to calculate the time elapsed AND organize it into the format [DD:]HH:MM:SS.0MS (days, hours, minutes, seconds, milliseconds). That being said, it is designed to balance precision and efficiency.

 - To use this as countdown timer, it is suggested to have some sort of alert command executed immediately after this program.
>As an example, using the bell character in a bash-like environment (might not be supported everywhere):
>
>`sleepview -m 1.5 && echo '\a'`
>
>Or
>
>`sleepview -m 1 30 && echo '\a'`
>
>Or
>
>`sleepview -t 1:30 && echo '\a'`
>
>Or
>
>`sleepview 1m 30s && echo '\a'` ('s' is optional here)
>
>This sets a timer for one and a half minutes, and prints the bell character to standard output assuming that it is not cancelled. If the syntax is wrong or the program is interrupted with ctrl-c or the kill command, it should not continue to the `echo` command due to the `&&` between the commands. In the case of incorrect syntax, the program panics internally and throws a `101` exit code as opposed to the success code of `0` which is required to continue past the `&&`. This is important, because you (probably) wouldn't want your alarm / chime to sound before the correct time.
>
>To use it regularly, a shell function along the lines of this might be in order:
>>```
>>function timer() {
>>    sleepview $@ && your_alarm_command
>>}
>>```

## Planned Features/Updates
 - [x] Add ability to parse arguments in the same way as GNU sleep, that is, `sleepview NUMBER[SUFFIX]` where the suffix can be nothing/'s' for seconds, 'm' for minutes, 'h' for hours, and 'd' for days.
 - [x] Add json output support, for better program interoperability.
 - [x] Add more options to adjust output. (Added -o, -r, -u, -R)
 - [ ] Improve error-handling for cases where either writing to stdout or writing to a file is restricted/disabled.

## Development
If you have cloned the repo, debug information can be enabled by setting the environment variable `RUST_LOG` to `debug`. e.g. `RUST_LOG=debug cargo run -- 1.1`

### Feedback
Relevant suggestions, questions, concerns, and/or issues may be directed to iedevfeedback@gmail.com

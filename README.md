Have you ever used the sleep command, only to be left wishing you could actually see the time counting down in real time? Well, now you can!

# Usage

>`sleepview [SWITCH] DURATION`

>DURATION: the amount of time to count down in seconds. Can be specified in combination with switches, or omitted entirely with switches present. Using a timestamp disables other switches.

>SWITCHES:

>-h :	Show this help message and exit.

>-d :	Specify days.

>-H :	Specify hours.

>-m :	Specify minutes.

>-t :   Specify a timestamp, in the form (D)D:(H)H:(M)M:(S)S(.DEC) -- days, hours, minutes, seconds, decimal portion.

# Installation

`cargo install sleepview`
(Requires rust, of course.)

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
>Or
>`sleepview -m 1 30 && echo '\a'`
>
>This sets a timer for one and a half minutes, and prints the bell character to standard output assuming that it is not cancelled. If the syntax is wrong or the program is interrupted with ctrl-c or the kill command, it should not continue to the `echo` command due to the `&&` between the commands. In the case of incorrect syntax, the program panics internally and throws a `101` exit code as opposed to the success code of `0` which is required to continue past the `&&`. This is important, because you (probably) wouldn't want your alarm / chime to sound before the correct time.
>
>To use it regularly, a shell function along the lines of this might be in order:
>>```
>>function timer() {
>>    sleepview "$@" && your_alarm_command
>>}
>>```

## Development
If you have cloned the repo, debug information can be enabled by setting the enviroment variable `RUST_LOG` to `debug`. E.g. `RUST_LOG=debug cargo run -- 1.1`

### Feedback
Relevant suggestions, questions, concerns, and/or issues may be directed to iedevfeedback@gmail.com

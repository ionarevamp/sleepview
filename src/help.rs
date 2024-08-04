pub const HELP_MSG: &str = "Usage: `sleepview [OPTIONS] [SWITCH] DURATION ...` or `sleepview [OPTIONS] DURATION[SUFFIX]...`

 DURATION: the amount of time to count down in seconds. Can be specified in combination with switches, or omitted entirely with switches present. Using a timestamp disables other switches, and only one of each other switch is allowed. Multiple non-timestamp durations will be added together.

 SUFFIX: can be 's', 'm', 'h', or 'd' for seconds, minutes, hours or days. Multiple durations of any kind will be added together. This is considered a fallback method, and only works properly without switches present.

 SWITCHES:
-h :\tShow this help message and exit.
-d :\tSpecify days.
-H :\tSpecify hours.
-m :\tSpecify minutes.
-t :\tSpecify a timestamp, in the form (D)D:(H)H:(M)M:(S)S(.DEC) -- days, hours, minutes, seconds, decimal portion.

 OPTIONS:
-f :\t(full) Show full width of timestamp, regardless of target time. Without this option, fields in the display format that will always show zero will be omitted.
-n :\t(no_newline) Do not append a new line when the program finishes naturally -- this generally causes the output to be overwritten by either the prompt or any other output on the same line as the countdown output.
-j :\t(json) Output data as json. Not recommended for normal use. Compatible with -f option.";





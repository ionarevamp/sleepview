pub const HELP_MSG: &str = "Usage: `sleepview [OPTIONS] {[SWITCH] DURATION}...` or `sleepview [OPTIONS] DURATION[SUFFIX]...`

 DURATION: the amount of time to count down in seconds. Can be specified in combination with switches, or omitted entirely with switches present. Using a timestamp disables other switches, and only one of each other switch is allowed. Multiple non-timestamp durations will be added together.

 SUFFIX: can be 's', 'm', 'h', or 'd' for seconds, minutes, hours or days. Multiple durations of any kind will be added together. This is considered a fallback method, and only works properly without switches present.

 SWITCHES:
-h :\tShow this help message and exit.
-d :\tSpecify days.
-H :\tSpecify hours.
-m :\tSpecify minutes.
-t :\tSpecify a timestamp, in the form (D)D:(H)H:(M)M:(S)S(.DEC) -- days, hours, minutes, seconds, decimal portion.

 OPTIONS:
   Note: use short options only.

-f :\t(full, boolean) Show full width of timestamp, regardless of target time. Without this option, fields in the display format that will always show zero will be omitted.
-n :\t(no_newline, boolean) Do not append a new line when the program finishes naturally -- this generally causes the output to be overwritten by either the prompt or any other output on the same line as the countdown output.
-j :\t(json, boolean) Output data as json. Not recommended for normal use. Compatible with -f option.
-o :\t(output, string) Specify an output file path, either absolute or relative to the current working directory. Can also be `-` for standard output, which is the default when no output path is specified.
-u :\t(up, boolean) Count up from 0 to the target time, instead down from the target time to 0.
-r :\t(rate, numeric) The minimum amount of time to wait as calculated by ( 1 / RATE ) seconds, if the amount of time left to count is greater than the calculated value. Otherwise, the amount of time between loops is cut to maintain accuracy to the specified target time.
-R :\t(resolution, character/integer) Specify the resolution of the output format. Applies to both json and default formats. Can be one of \"m\" or 0 (milliseconds), \"s\"/\"S\" or 1 (seconds), \"M\" or 2 (minutes), \"h\"/\"H\" or 3 (hours), OR \"d\"/\"D\" or 4 (days). This omits the number of fields specified, and so should not attempt to omit more fields than would be displayed without the -f flag.
-I :\t(instant, boolean) By default, as of version 2.0.3, sleepview uses the SystemTime API to record when the program started. If you choose to use the Instant API instead, then be warned that system idle periods may skew results.
";

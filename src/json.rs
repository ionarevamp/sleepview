// format_width should be passed in as num_fields
use std::io::Write;

#[inline(always)]
pub fn produce_json(values: [i128; 5], num_fields: usize, resolution: usize, mut output: impl Write) {
    let _ = write!(output, "{{");
    let mut field_count = num_fields;
    for val in 
        values[resolution..num_fields].iter().rev()
        {
            match field_count-1 {
                0 => {
                    let _ = write!(output, " \"milliseconds\": \"{}\"", val);
                },
                1 => {
                    let _ = write!(output, " \"seconds\": \"{}\"", val);
                },
                2 => {
                    let _ = write!(output, " \"minutes\": \"{}\"", val);
                },
                3 => {
                    let _ = write!(output, " \"hours\": \"{}\"", val);
                },
                4 => {
                    let _ = write!(output, " \"days\": \"{}\"", val);
                },
                _ => {
                    let _ = write!(output, " \"unknown_field\": \"{}\"", val);
                }
            }

            if field_count-resolution > 1 {
                let _ = write!(output, ",");
            } else {
                let _ = write!(output, " ");
            }
            field_count -= 1;
        }
    let _ = write!(output, "}}");
}

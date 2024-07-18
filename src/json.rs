// format_width should be passed in as num_fields

pub fn produce_json(values: [i128; 5], num_fields: usize) {
    print!("{{");
    let mut field_count = num_fields;
    for val in 
        values[..num_fields].iter().rev()
        {
            match field_count-1 {
                0 => {
                    print!(" \"milliseconds\": \"{}\"", val);
                },
                1 => {
                    print!(" \"seconds\": \"{}\"", val);
                },
                2 => {
                    print!(" \"minutes\": \"{}\"", val);
                },
                3 => {
                    print!(" \"hours\": \"{}\"", val);
                },
                4 => {
                    print!(" \"days\": \"{}\"", val);
                },
                _ => {
                    print!(" \"unknown_field\": \"{}\"", val);
                }
            }

            if field_count > 1 {
                print!(",");
            } else {
                print!(" ");
            }
            field_count -= 1;
        }
    print!("}}");
}

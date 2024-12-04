use std::fs::read_to_string;
fn get_data () -> String {
    const FILENAME: &str = "data";
    read_to_string(FILENAME)
        .unwrap()

}
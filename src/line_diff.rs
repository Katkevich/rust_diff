pub struct LineDiff {
    intervals: Vec<(i32, i32)>
}

impl LineDiff {
    pub fn new() -> LineDiff {
        LineDiff {
            intervals: { Vec::<(i32, i32)>::new() }
        }
    }
}

use line_diff::{ LineDiff };

pub enum Diff {
    NoChanges,
    Added,
    Removed,
    Changed(LineDiff),
}

use ariadne::{Report, ReportKind, Label, Source, Color, ColorGenerator, Fmt};

fn main() {
    let mut colors = ColorGenerator::new();

    // Generate & choose some colours for each of our elements
    let a = colors.next();
    let b = colors.next();
    let out = Color::Fixed(81);

    Report::build(ReportKind::Error, "sample.tao", 12)
        .with_code(3)
        .with_message(format!("Incompatible types"))
        .with_label(Label::new(("sample.tao", 32..33))
            .with_message(format!("This is of type {}", "Nat".fg(a)))
            .with_color(a))
        .with_label(Label::new(("sample.tao", 42..45))
            .with_message(format!("This is of type {}", "Str".fg(b)))
            .with_color(b))
        .with_label(Label::new(("sample.tao", 11..48))
            .with_message(format!(
                "The values are outputs of this {} expression",
                "match".fg(out),
            ))
            .with_color(out))
        .with_note(format!("Outputs of {} expressions must coerce to the same type", "match".fg(out)))
        .finish()
        .print(("sample.tao", Source::from(include_str!("sample.tao"))))
        .unwrap();
}
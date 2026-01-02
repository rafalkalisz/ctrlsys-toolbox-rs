pub fn tf_text(num: &[f64], den: &[f64]) -> String {

    let num_str = poly_text(num);
    let den_str = poly_text(den);
    let width = num_str.len().max(den_str.len());

    let centered = |s: &str| {
        let padding = (width.saturating_sub(s.len())) / 2;
        format!("{:padding$}{}", "", s, padding = padding)
    };

    format!(
        "{}\n{}\n{}",
        centered(&num_str),
        "-".repeat(width),
        centered(&den_str),
    )

}

fn poly_text(coeffs: &[f64]) -> String {
    let degree = coeffs.len().saturating_sub(1);
    let mut terms = vec![];

    for (i, &c) in coeffs.iter().enumerate() {
        let power = degree - i;
        if c.abs() < 1e-12 {
            continue;
        }
        let term = match power {
            0 => format!("{:.3}", c),
            1 => format!("{:.3}s", c),
            _ => format!("{:.3}s^{}", c, power),
        };
        terms.push(term);
    }

    terms.join(" + ")
}

pub fn print_coeffs(coeffs: &[f64]) -> String {
    let mut output = String::new();
    for (i, &c) in coeffs.iter().enumerate() {
        if i > 0 {
            output.push_str(", ");
        }
        output.push_str(&format!("{:.6}", c));
    }
    output
}
use rand::Rng;

pub fn boolean_expression() -> String {
    let mut rng = rand::rng();

    let a = rng.random_range(1..=99);
    let b = rng.random_range(1..=99);

    let operator = if rng.random_bool(0.5) {
        "AND"
    } 
    else {
        "OR"
    };

    let expression = format!("{} b{} b{}", operator, a, b);

    // Determines whether NOT is added
    if rng.random_bool(0.3) {
        format!("NOT ({})", expression)
    } else {
        expression
    }

}

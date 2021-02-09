let number = params[0];

if number < 1000 {
    number
} else {
    (number.to_float() / 100.0).round() / 10.0 + "k"
}

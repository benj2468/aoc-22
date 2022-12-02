pub fn run_ty() -> String {
    std::env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("sample"))
}

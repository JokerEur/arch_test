use some_module::PatternType;

fn check(val: PatternType) {
    match val {
        PatternType::VariantA => {}
        PatternType::VariantB => {}
    }
}
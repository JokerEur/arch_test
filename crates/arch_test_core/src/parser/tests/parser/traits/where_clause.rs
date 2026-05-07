use some_module::BoundType;

fn generic_fn<T>(val: T)
where
    T: BoundType,
{
}
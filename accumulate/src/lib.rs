/// What should the type of _function be?
pub fn map<Got, Out>(
    input: Vec<Got>, 
    mut _function: impl FnMut(Got)->Out
) -> Vec<Out> {
    //unimplemented!("Transform input vector {input:?} using passed function");
    let mut result = Vec::with_capacity(input.len());
    for i in input {
        result.push(_function(i));
    }
    result
}

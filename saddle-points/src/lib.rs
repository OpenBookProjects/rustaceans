pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    //unimplemented!("find the saddle points of the following matrix: {input:?}")
    let mut result=Vec::new();
    for (y,col)in input.iter().enumerate(){
        for (x,row) in col.iter().enumerate(){
            if col.iter().all(|v| row>=v)&&(0..input.len()).all(|i| row<=&input[i][x]){
                result.push((y,x));
            }
        }
    }
    result
}

//pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    //unimplemented!("Function that returns the spiral matrix of square size {size}");
    let mut sm = vec![vec![0;size as usize];size as usize];
    let (mut i,mut j):(i32,i32) = (0,-1);
    let (mut di,mut dj):(i32,i32) = (0,1);
    let mut n = 0;
    let mut top_limit = size;
    
    while n < size*size{
        for _ in 0..top_limit{
            i+=di;
            j+=dj;
            n+=1;
            sm[i as usize][j as usize] = n;
        }
        if dj.abs() == 1{
            top_limit -=1;
        }
        (di,dj)=(dj,-di)
    }
    sm

}


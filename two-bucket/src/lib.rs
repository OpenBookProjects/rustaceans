#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// greatest common divisor
fn gcd(a: u8, b: u8) -> u8 {
    if b == 0{
        a
    }else{
        gcd(b, a % b)
    }
}

/// Solve the bucket problem
pub fn solve(
    //capacity_1: u8,
    //capacity_2: u8,
    left: u8,
    right: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
/*     unimplemented!(
        "Given one bucket of capacity {capacity_1}, another of capacity {capacity_2}, starting with {start_bucket:?}, find pours to reach {goal}, or None if impossible"
    );
 */
    if goal % gcd(left,right) !=0{
        return None;
    }
    let (mut state, limits, bucket) = match start_bucket {
        Bucket::One => (
            [0,left], 
                (right,left), 
                (Bucket::Two,Bucket::One)
            ),
        Bucket::Two => (
            [0,right], 
                (left, right), 
                (Bucket::One,Bucket::Two)
        ),
    };
    
    let mut moves=1;

    while goal != state[0] && goal != state[1] {
        if state[0] == 0 && limits.0 == goal{
            state[0] = goal;
        } else if state[1] == 0 && limits.1 == goal{
            state[1] = goal;
        } else if state[0] == limits.0 {
            state[0] = 0;
        }else if state[1] == 0 {
            state[1] = limits.1;
        }else{
            state=[
                limits.0.min(state.iter().sum()),
                state[1].checked_sub(limits.0 - state[0]).unwrap_or(0),
            ]
        }
        moves+=1;
    }

    Some( BucketStats{
        moves,
        goal_bucket: if state[0] == goal{
                bucket.0
            }else{
                bucket.1
            },
        other_bucket: if state[0] == goal{
                state[1]
            }else{
                state[0]
            },
    })
}


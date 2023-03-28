//use itertools::Itertools;

pub fn count(lines: &[&str]) -> u32 {
    let chars = lines
            .iter()
            .map(|l| l.as_bytes())
            .collect::<Vec<_>>();
    let width = lines
            .get(0)
            .map(|l| l.len())
            .unwrap_or_default();
    let height = lines.len();


    let mut count = 0;
    for top in 0..height
    {
        for left in 0..width
        {
            for bottom in top+1..height
            {
                for right in left+1..width
                {
                    if chars[top][left] == b'+'
                        && chars[top][right] == b'+'
                        && chars[bottom][left] == b'+'
                        && chars[bottom][right] == b'+'
                        && (top+1..bottom).all(|i| {
                            matches!(
                                (chars[i][left], chars[i][right]),
                                (b'|'| b'+', b'|'| b'+')
                            )})
                        && (left+1..right).all(|j| {
                            matches!(
                                (chars[top][j], chars[bottom][j]),
                                (b'-'| b'+', b'-'| b'+')
                            )})
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count 
    

}


/* 


    (2..height)
        .combinations(2)
        .map(|pair| {
            let (top, bottom) = (pair[0], pair[1]); // 解构元组
            vec![top, bottom].into_iter() // 返回一个包含两个元素的 Vec
        })
        .flat_map(|mut tb| {
            //let top = tb[0];
            //let bottom = tb[1];
            let top = tb.nth(0).unwrap();
            let bottom = tb.nth(0).unwrap();
            (0..width)
                .combinations(2)
                .map(move |c| (top, bottom, c[0], c[1]))
                .collect::<Vec<_>>()
                .into_iter()
        })
        .filter_map(|(top, bottom, left, right)| {
            if chars[top][left] == b'+'
                && chars[top][right] == b'+'
                && chars[bottom][left] == b'+'
                && chars[bottom][right] == b'+'
                && (top + 1..bottom).all(|i| {
                    matches!(
                        (chars[i][left], chars[i][right]),
                        (b'|' | b'+', b'|' | b'+')
                    )
                })
                && (left + 1..right).all(|j| {
                    matches!(
                        (chars[top][j], chars[bottom][j]),
                        (b'-' | b'+', b'-' | b'+')
                    )
                })
            {
                Some(1)
            } else {
                None
            }
        })
        .sum::<u32>()


    (2..height)
        .combinations(2)
        .flat_map(|(top, bottom)| {
            (0..width).combinations(2).map(move |c| (top, bottom, c[0], c[1])).collect::<Vec<_>>()
        })
        .filter_map(|(top, bottom, left, right)| {
    if chars[top][left] == b'+'
        && chars[top][right] == b'+'
        && chars[bottom][left] == b'+'
        && chars[bottom][right] == b'+'
        && (top+1..bottom).all(|i| {
            matches!(
                (chars[i][left], chars[i][right]),
                (b'|'| b'+', b'|'| b'+')
            )})
        && (left+1..right).all(|j| {
            matches!(
                (chars[top][j], chars[bottom][j]),
                (b'-'| b'+', b'-'| b'+')
            )})
    {
        Some(1)
    } else {
        None
    }
})
    

    (2..height)
        .combinations(2)
        .map(|(top, bottom)| {
            (0..width)
                .combinations(2)
                .map(move |c| (top, bottom, c[0], c[1]))
        })
        .flatten()
        .filter_map(|(top, bottom, left, right)| {
            if chars[top][left] == b'+'
                && chars[top][right] == b'+'
                && chars[bottom][left] == b'+'
                && chars[bottom][right] == b'+'
                && (top + 1..bottom).all(|i| {
                    matches!(
                        (chars[i][left], chars[i][right]),
                        (b'|' | b'+', b'|' | b'+')
                    )
                })
                && (left + 1..right).all(|j| {
                    matches!(
                        (chars[top][j], chars[bottom][j]),
                        (b'-' | b'+', b'-' | b'+')
                    )
                })
            {
                Some(1)
            } else {
                None
            }
        })
        .sum()


    (2..height)
        .combinations(2)
        .flat_map(|(top, bottom)| {
            (0..width)
                .combinations(2)
                .map(move |c| vec![top, bottom, c[0], c[1]])
                .collect::<Vec<_>>()
        })
        .filter_map(|v| {
            let (top, bottom, left, right) = (v[0], v[1], v[2], v[3]);
            if chars[top][left] == b'+'
                && chars[top][right] == b'+'
                && chars[bottom][left] == b'+'
                && chars[bottom][right] == b'+'
                && (top + 1..bottom).all(|i| {
                    matches!(
                        (chars[i][left], chars[i][right]),
                        (b'|' | b'+', b'|' | b'+')
                    )
                })
                && (left + 1..right).all(|j| {
                    matches!(
                        (chars[top][j], chars[bottom][j]),
                        (b'-' | b'+', b'-' | b'+')
                    )
                })
            {
                Some(1)
            } else {
                None
            }
        })
        .sum()


    (2..height)
        .combinations(2)
        .map_collect(|(top, bottom)| vec![top, bottom])
        .map(|(top, bottom)| vec![top, bottom])
        //.map(|(top, bottom)| vec![top, bottom].into_iter().collect::<Vec<_>>())
        .flat_map(|tb| {
            let top = tb[0];
            let bottom = tb[1];
            (0..width)
                .combinations(2)
                .map(move |c| (top, bottom, c[0], c[1]))
                .collect::<Vec<_>>()
        })
        .filter_map(|(top, bottom, left, right)| {
            if chars[top][left] == b'+'
                && chars[top][right] == b'+'
                && chars[bottom][left] == b'+'
                && chars[bottom][right] == b'+'
                && (top + 1..bottom).all(|i| {
                    matches!(
                        (chars[i][left], chars[i][right]),
                        (b'|' | b'+', b'|' | b'+')
                    )
                })
                && (left + 1..right).all(|j| {
                    matches!(
                        (chars[top][j], chars[bottom][j]),
                        (b'-' | b'+', b'-' | b'+')
                    )
                })
            {
                Some(1)
            } else {
                None
            }
        })
        .sum::<u32>()


*/


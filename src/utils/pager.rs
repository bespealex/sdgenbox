pub fn pager(count: u32, current: u32, on_each_side: u32, on_ends: u32) -> Vec<Option<u32>> {
    if count <= (on_each_side + on_ends) * 2 {
        return (1..(count + 1)).map(Some).collect();
    }

    let mut result = vec![];
    if current > (1 + on_each_side + on_ends) + 1 {
        result.extend((1..(on_ends + 1)).map(Some));
        result.push(None);
        result.extend(((current - on_each_side)..(current + 1)).map(Some))
    } else {
        result.extend((1..(current + 1)).map(Some));
    }

    if current < (count - on_each_side - on_ends) - 1 {
        result.extend(((current + 1)..(current + on_each_side + 1)).map(Some));
        result.push(None);
        result.extend(((count - on_ends + 1)..(count + 1)).map(Some))
    } else {
        result.extend(((current + 1)..(count + 1)).map(Some));
    }

    result
}

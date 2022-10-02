fn next_pow2(n: usize) -> usize {
    let mut n = n - 1;
    let mut i = 0;

    while i <= 4 {
        n |= n >> 2u8.pow(i);
        i += 1;
    }

    n + 1
}

fn shortest_be_bytes(d: usize) -> Vec<u8> {
    let mut np2 = next_pow2(d);
    let mut exp = 1; // exponent

    // The representation will always take at least one byte.
    if np2 < 2 {
        np2 = 2;
    }

    // Get the pow of a number.
    // This counts the number of bits required to represent a number up to the least significant
    // bit, but does not include it:
    // 8 4 2 1
    // 1 1 1 0
    // The code accounts for the first bit in the line where `pow` is divided by 8.
    while np2 != 2 {
        np2 /= 2;
        exp += 1;
    }

    // This might overflow and panic.
    // Adding one to `pow` to account for a first bit.
    exp = (exp + 1 + 8 - 1) / 8;

    let mut res = vec![0u8; exp];

    // Inserting the least significant byte to the front of res.
    for (i, b) in d.to_le_bytes().iter().enumerate() {
        if i > exp - 1 {
            break;
        }
        res[i] = *b;
    }

    // Reversing res to have most significant bytes at the front.
    res.reverse();
    res
}

fn find_repeat_element(search_buf: &[u8], data: &[u8]) -> Option<(usize, usize)> {
    let mut coords = None;
    let mut length = 0;
    let mut dist = 0;

    for (i, sb) in search_buf.iter().enumerate() {
        if data[length] == *sb {
            length += 1;
            dist = i;
        }
    }

    if length != 0 && dist != 0 {
        coords = Some((length, dist));
    }

    coords
}

fn to_code(length: usize, distance: usize) -> Vec<u8> {
    let mut ld = shortest_be_bytes(length);
    ld.append(&mut ":".as_bytes().to_vec());
    ld.append(&mut shortest_be_bytes(distance));

    ld
}

fn find_repeat_elements(data: &[u8]) -> Vec<u8> {
    // init the search buffer
    let mut search_buf = Vec::default();
    let mut current_buf = Vec::default();

    let mut out = Vec::default();
    // loop overdata
    for b in data.iter() {
        // append every char into search buffer
        search_buf.push(*b);
        current_buf.push(*b);

        // iterate over elements in the `current buffer to compress`
        if let Some((l, d)) = find_repeat_element(&search_buf, &current_buf) {
            let _ = to_code(l, d);
            //out.append(&mut to_code(l, d));
            //current_buf.clear();
        };
    }

    out
}

mod tests {
    use crate::find_repeat_element;

    #[test]
    fn test_to_code() {
        use crate::to_code;

        let (length, distance) = (256, 20);
        let coord = to_code(length, distance);

        // `256:20`.
        assert_eq!(coord, &[1, 0, 58, 20]);

        assert_eq!(to_code(1, 1), &[1, 58, 1]);
    }

    #[test]
    fn test_find_repeat() {
        use crate::find_repeat_element;

        let data = [10, 20, 30, 40, 10, 20, 30, 40, 50];
        let buf = [10, 20, 30, 40];
        let coords = find_repeat_element(&data, &buf);

        assert_eq!(coords, Some((4, 4)));
    }

    #[test]
    fn test_repeat() {
        use super::*;

        let data = [10, 20, 30, 40, 10, 20, 30, 40, 50];
        let expected = [10, 20, 30, 40, 4, 58, 4, 50];
        let compressed = find_repeat_elements(&data);

        assert_eq!(compressed, expected);
    }
}

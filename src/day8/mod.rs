#[derive(Debug)]
enum Error {
    InvalidInput,
}

const SIZE: (usize, usize) = (25, 6);

fn gen_layers(input: &str) -> crate::Result<Vec<Vec<u32>>> {
    let mut layers: Vec<Vec<u32>> = Vec::new();
    let mut chars = input.chars().peekable();
    while chars.peek().is_some() {
        layers.push(chars.by_ref().take(SIZE.0 * SIZE.1).map(|c| {
            c.to_digit(10)
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))
        }).collect::<Result<_, _>>()?);
    }

    // Check that all layers have the proper size
    if layers.iter().any(|l| l.len() != SIZE.0 * SIZE.1) {
        Err(crate::Error::boxed(Error::InvalidInput))
    } else {
        Ok(layers)
    }
}

pub fn part1(input: &str) -> crate::Result<usize> {
    let l = gen_layers(input)?
        .into_iter()
        .min_by_key(|l| l.iter().filter(|&&v| v == 0).count())
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;

    Ok(l.iter().filter(|&&v| v == 1).count()
        * l.iter().filter(|&&v| v == 2).count())
}

pub fn part2(input: &str) -> crate::Result<String> {
    let layers = gen_layers(input)?;
    let mut img = String::new();
    img.push('\n');
    for y in 0..SIZE.1 {
        for x in 0..SIZE.0 {
            match layers
                .iter()
                .map(|l| l[x + y * SIZE.0])
                .find(|&v| v != 2)
                .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?
            {
                0 => img.push(' '),
                1 => img.push('#'),
                _ => return Err(crate::Error::boxed(Error::InvalidInput)),
            }
        }
        img.push('\n');
    }

    Ok(img)
}

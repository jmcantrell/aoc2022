use anyhow::{anyhow, Context};

use super::{Parsed1, Parsed2};

pub type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(output: &Parsed1) -> anyhow::Result<Solution1> {
    let file_system = output.reconstruct_file_system()?;

    let sizes: Vec<_> = file_system
        .directory_sizes()
        .filter(|&&size| size <= 100_000)
        .collect();

    if sizes.len() > 0 {
        Ok(sizes.into_iter().sum())
    } else {
        Err(anyhow!("no directories found"))
    }
}

pub fn solve2(output: &Parsed2) -> anyhow::Result<Solution2> {
    let file_system = output.reconstruct_file_system()?;

    let capacity: usize = 70_000_000;
    let available = capacity - file_system.size();

    let required: usize = 30_000_000;
    let desired = required - available;

    let sizes: Vec<_> = file_system
        .directory_sizes()
        .filter(|&&size| size >= desired)
        .collect();

    sizes
        .into_iter()
        .min()
        .cloned()
        .context("no directories found")
}

#[cfg(test)]
pub mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 95437);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 24933642);
        Ok(())
    }
}

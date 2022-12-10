use crate::command::Command;
use crate::ship::Stacks;
use anyhow::Context;

pub trait Crane {
    fn move_crates(stacks: &mut Stacks, count: usize, from: usize, to: usize)
        -> anyhow::Result<()>;

    fn execute_command(stacks: &mut Stacks, command: &Command) -> anyhow::Result<()> {
        match command {
            Command::Move { count, from, to } => {
                Self::move_crates(stacks, *count, *from - 1, *to - 1)
            }
        }
    }
}

pub struct Crane9000;

impl Crane for Crane9000 {
    fn move_crates(
        stacks: &mut Stacks,
        count: usize,
        from: usize,
        to: usize,
    ) -> anyhow::Result<()> {
        for i in 0..count {
            let c = stacks.pick_up(from).with_context(|| {
                format!(
                    "stack number {} is empty on iteration number {}",
                    from + 1,
                    i + 1
                )
            })?;
            stacks.put_down(to, c);
        }

        Ok(())
    }
}

pub struct Crane9001;

impl Crane for Crane9001 {
    fn move_crates(
        stacks: &mut Stacks,
        count: usize,
        from: usize,
        to: usize,
    ) -> anyhow::Result<()> {
        let mut buffer: Vec<_> = Default::default();

        for i in 0..count {
            buffer.push(stacks.pick_up(from).with_context(|| {
                format!(
                    "stack number {} is empty on iteration number {}",
                    from + 1,
                    i + 1
                )
            })?);
        }

        while !buffer.is_empty() {
            stacks.put_down(to, buffer.pop().unwrap());
        }

        Ok(())
    }
}

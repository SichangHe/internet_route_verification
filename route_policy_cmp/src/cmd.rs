use std::{
    io::{BufReader, Result},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

pub struct PipedChild {
    child: Child,
    pub stdin: ChildStdin,
    pub stdout: BufReader<ChildStdout>,
}

impl PipedChild {
    pub fn new(command: &mut Command) -> Result<Self> {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        Ok(Self {
            child,
            stdin,
            stdout,
        })
    }
}

impl Drop for PipedChild {
    fn drop(&mut self) {
        _ = self.child.kill();
    }
}

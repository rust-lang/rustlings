use anyhow::{bail, Result};
use std::io::{stdout, Write};

use crate::exercise::Exercise;

// Invoke the rust compiler on the path of the given exercise,
// and run the ensuing binary.
// The verbose argument helps determine whether or not to show
// the output from the test harnesses (if the mode of the exercise is test)
pub fn run(exercise: &Exercise) -> Result<()> {
    let output = exercise.run()?;

    {
        let mut stdout = stdout().lock();
        stdout.write_all(&output.stdout)?;
        stdout.write_all(&output.stderr)?;
        stdout.flush()?;
    }

    if !output.status.success() {
        bail!("Ran {exercise} with errors");
    }

    success!("Successfully ran {}", exercise);

    Ok(())
}

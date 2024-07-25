# SOSIX Utils

The **SOS Interface** utils!

<br>

We don't want more of the *same*.

We want *practicality*, *simplicity*, *correctness*, and *reliability*.

## Reason

These utilities are simply extraordinarily common CLI/shell commands to further
simplify a User-centric Operating System.

These are the kinds of commands a range of CLI'ers execute 1500 times a day.

These commands are stupid simple to use and don't have any complex flags. Just
straightforward arguments and reliable behavior.

Heck the implementation for most of the commands hardly exceeds 50 lines!

# Scope

## Commands

- `pwd` (**DONE**)
- `cp`
  - files
    - `cp ${source_file} ${destination_file}` (**DONE**)
    - `cp ${source_file} ${destination_folder}`
  - directories
    - `cp ${source_folder} ${destination_folder}`
- `mv`
  - files
    - `mv ${source_file} ${destination_file}` (**DONE**)
    - `mv ${source_file} ${destination_folder}`
  - directories
    - `mv ${source_folder} ${destination_folder}`
- `ls` (**DONE**)
- `stat` (**DONE**)
- `touch` (**DONE**)
- `rm` (**DONE**)
- `mkdir` (**DONE**)
- `rmdir` (**DONE**)
- `trash`
  - `macos` (**DONE**)
    - Ugly because Apple can't figure out how to move items to the user's trash bin
  - `windows`
  - `linux`
- `co` (`chown`) (**DONE**)
- `cm` (`chmod`)
- `sl` ((soft) `link`)
- `df`
- `du`
- `env` ?
- `cat` ?
- `type` ?
- `command`/`\` ?
- `ping` ?
- `sh`
  - `.`
  - `cd`
  - `echo`
  - `exit`

- ...and probably more to come!

## What SOSIX Utils is *NOT*

Overall, we want to keep the names of commands where possible. But SOSIX is not trying to be POSIX.
The primary goal is usability and intuitive behavior. No surprises. If you type in a command to
`rmdir` - its going to remove that directory, no questions or options asked.

In keeping with simplicity, oftentimes that means accepting that two-, three-, somtimes five-letter
commands are exceedingly easy to remember.

This won't be an attempt at porting all POSIX commands. There are over a
hundred POSIX-defined standard commands. Around 10 of them are used multiple
times every single day. Those are the commands being targeted by SOSIX.

It won't even be an attempt to respect any POSIX command flags.

Extensive commands that have their own scripting language (looking at *you*,
`awk`...) have no place here.

SOSIX utils has no interest in providing opportunities for its ecosystem to
become Turing-complete. There is no desire to be able to perform advanced
scripting with functions and variables and the like using SOSIX utils.

The commands are for interactive engagement with a computer. If you want a
scripting shell take a look at `python` or some such.

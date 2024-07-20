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
- `cp` (**DONE**)
- `mv` (**DONE**)
- `ls` (**DONE**)
- `stat` (**DONE**)
- `mkdir` (**ON DECK**)
- `rm` (**ON DECK**)
- `env` ?
- `cat` ?
- `sh`
  - `.`
  - `cd`
  - `echo`
  - `exit`

- ...and probably more to come!

## Notes

Overall, we want to keep the names of commands as conventional as possible.

In keeping with simplicity, oftentimes that means accepting that

1) muscle-memory is a thing
1) two-, three-, somtimes five-letter commands are exceedingly easy to remember
1) two-, three-, somtimes five-letter commands are exceedingly easy to punch
into a prompt

## What SOSIX Utils is *NOT*

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

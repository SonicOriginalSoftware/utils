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

Heck, with the help of `rust`'s `std`, the implementations for most of the
commands hardly exceed 50 lines!

## Examples

**The formatting below is by design; no extra arguments needed to sort output
or give column widths**

```sh
> ls /

drwxr-xr-x    64 B      root          wheel      .vol
drwxrwxr-x   864 B      root          admin      Applications
drwxr-xr-x   1.2 K      root          wheel      bin
drwxrwxr-t    64 B      root          admin      cores
dr-xr-xr-x   4.6 K      root          wheel      dev
drwxr-xr-x   2.2 K      root          wheel      Library
drwxr-xr-x    96 B      root          wheel      opt
drwxr-xr-x   192 B      root          wheel      private
drwxr-xr-x   2.0 K      root          wheel      sbin
drwxr-xr-x   320 B      root          wheel      System
drwxr-xr-x   192 B      root          admin      Users
drwxr-xr-x   352 B      root          wheel      usr
drwxr-xr-x    96 B      root          wheel      Volumes
----------     0 B      root          admin      .file
lrwxr-xr-x    36 B      root          admin      .VolumeIcon.icns     -> System/Volumes/Data/.VolumeIcon.icns
lrwxr-xr-x    11 B      root          wheel      etc                  -> private/etc
lrwxr-xr-x    25 B      root          wheel      home                 -> /System/Volumes/Data/home
lrwxr-xr-x    11 B      root          wheel      tmp                  -> private/tmp
lrwxr-xr-x    11 B      root          wheel      var                  -> private/var

> ls /var/run

drwxr-xr-x    96 B  _assetcache    _assetcache   com.apple.AssetCache
drwx---r-x   160 B      root          daemon     com.apple.security.cryptexd
drwxr-xr-x    64 B      root          daemon     com.apple.wifivelocity
drwx------    96 B      root          daemon     mds
-rw-------     0 B      root          daemon     automount.initialized
-rw-r--r--     6 B      root          daemon     bootpd.pid
-r--------     0 B      root          daemon     com.apple.DumpPanic.finishedPMUFaultHandling
-r--------     0 B      root          daemon     com.apple.DumpPanic.finishedThisBoot
-r--------     0 B      root          daemon     com.apple.logind.didRunThisBoot
-r--------     0 B      root          daemon     com.apple.loginwindow.didRunThisBoot
-r--------     0 B      root          daemon     com.apple.mdmclient.daemon.didRunThisBoot
-rw-r--r--   405 B      root          daemon     com.apple.mobileassetd-MobileAssetBrain
-rw-r--r--     0 B      root          daemon     com.apple.softwareupdate.availableupdatesupdated
----------     0 B      root          daemon     com.apple.WindowServer.didRunThisBoot
-rw-r--r--     4 B      root          daemon     diskarbitrationd.pid
-rw-r--r--     6 B      root          daemon     hdiejectd.pid
-rw-r--r--   374 B      root          daemon     resolv.conf
-rw-r--r--     4 B      root          daemon     syslog.pid
-r--r--r--     0 B      root          daemon     systemkeychaincheck.done
-rw-r--r--   3.1 K      root          daemon     utmpx
-rw-r--r--     0 B      root          wheel      wifi
srwxrwxrwx     0 B      root          daemon     .sim_diagnosticd_socket
srwxrwxrwx     0 B      root          daemon     cupsd
srw-r--r--     0 B  thethinkerer      wheel      filesystemui.socket
srw-rw-rw-     0 B      root          daemon     mDNSResponder
srwxrwxrwx     0 B      root          daemon     portmap.socket
srwxrwxrwx     0 B      root          daemon     pppconfd
srw-rw-rw-     0 B      root          daemon     syslog
srw-rw-rw-     0 B      root          daemon     systemkeychaincheck.socket
srwxrwxrwx     0 B      root          daemon     usbmuxd
srw-------     0 B      root          daemon     vpncontrol.sock
```

# Scope

## Commands

- [x] `pwd`
- `cp`
  - files
    - [x] `cp ${source_file} ${destination_file}`
    - [x] `cp ${source_file} ${destination_folder}`
  - directories
    - [ ] `cp ${source_folder} ${destination_folder}`
- `mv`
  - files
    - [x] `mv ${source_file} ${destination_file}`
    - [x] `mv ${source_file} ${destination_folder}`
  - directories
    - [x] `mv ${source_folder} ${destination_folder}`
- [x] `ls`
- [x] `stat`
- [x] `touch`
- [x] `rm`
- [x] `mkdir`
- [x] `rmdir`
- `trash`
  - [x] `macos`
    - Ugly because Apple can't figure out how to move items to the user's trash
    bin
  - [ ] `windows`
  - [ ] `linux`
- [x] `co` (`chown`)
- [x] `env`
- [x] `cm` (`chmod`)
- [x] `cat`
- [x] `ln` ((soft) `link`)
- [ ] `kill`
- [ ] `command` ?
- [ ] `type` ?
- [ ] `which` ?
- [ ] `sh`
  - [ ] `.`
  - [ ] `cd`
  - [ ] `echo`
  - [ ] `alias`
  - [ ] `exit`
- [ ] `df` ?
- [ ] `du` ?
- [ ] `ping` ?

- ...and probably more to come!

## What SOSIX Utils is *NOT*

Overall, we want to keep the names of commands where possible. But SOSIX is not
trying to be POSIX. The primary goal is usability and intuitive behavior.
No surprises. If you enter the command to `rmdir` and pass it a valid
directory - its going to remove that directory, no questions or options asked.

In keeping with simplicity, oftentimes that means accepting that two-, three-,
somtimes five-letter commands are exceedingly easier to remember.

This won't be an attempt at porting all POSIX commands. It won't even be an
attempt to respect any POSIX command flags. There are over one hundred
POSIX-defined standard commands. Less than 50 of them are used multiple times
every single day. These are the commands being targeted by SOSIX.


Extensive commands that have their own scripting language (looking at *you*,
`awk`...) have no place here.

SOSIX `utils` have no interest in providing opportunities for its ecosystem to
become Turing-complete. There is no desire to be able to perform advanced
scripting (as powerful as `sh` can be) with functions and variables and the
like using SOSIX utils.

These commands are for human-interactive engagement with a computer. There are
far better interpreted languages for such tasks.

# Install

## From source

```
cargo install --path .
```

# Uninstall

## From source

```
cargo uninstall --package utils
```

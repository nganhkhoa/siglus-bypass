# Siglus Engine by pass

## Usage

1. Download release binary siglus-bypass.exe
2. Place it at the same place as SiglusEngine.exe
3. Run siglus-bypass.exe

## Why

Sometimes I download the game and not included patched SiglusEngine.exe. And I have to go find the
auto patch. The auto patch is also flagged as malware and I have to disable anti-malware program
(Windows Defender) to start. Time to time, I don't download the auto patch program anymore. I just
open disassembler and patch the program by myself. Now the point is I have to do it again from
stratch because I don't remember how to patch and because new file. Then I decided that I should
create myself a patcher. And I see falconre written in Rust. I just try writing one.

I don't intend to try all games. I only update if the code doesn't work for games I play.

## How

1. For each function, iterate all instruction and find all Store Constant
2. Read Constant at memory, if the utf16 string is "This Game is Japan only\n\n", return function
3. Find function file offset from virtual address
4. Fix bytes into `or al, 1; ret`

## Notes

The current falconre framework is far from complete, so some fixes are to be expected to make this
program works. Current fixes are listed below:

- Add `xorps` instruction. Minimal fix to make it able to find the check function.
- Add xmm0 registers to X86.
- Change Pe::pe() to public.

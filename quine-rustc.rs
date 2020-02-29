error: unknown start of token: `
 --> quine-rustc.rs:1:32
  |
1 | error: unknown start of token: `
  |                                ^
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
1 | error: unknown start of token: '
  |                                ^

error: unknown start of token: `
 --> quine-rustc.rs:4:36
  |
4 | 1 | error: unknown start of token: `
  |                                    ^
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
4 | 1 | error: unknown start of token: '
  |                                    ^

error: character constant must be escaped: '
 --> quine-rustc.rs:6:56
  |
6 | help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |                                                        ^

error: aborting due to 3 previous errors


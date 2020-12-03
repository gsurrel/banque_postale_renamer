# Banque Postale Renamer

This utility is useful for renaming one or several files from La Banque Postale. This requires a `rust` toolchain. With `cargo`, you simply need the following command to batch-process a full directory (recursively):

    cargo run -- path/to/directory

More information:

```sh
$ banque_postale_renamer -h
banque_postale_renamer 1.0
Grégoire Surrel (https://gregoire.surrel.org)
Renames all recognized PDF files from La Banque Postale according to their contents

USAGE:
banque_postale_renamer <INPUT>

ARGS:
<INPUT>    Sets the input file or folder to use

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information
```

[A more detailed article (in French) about this tool is available on my website](https://gregoire.surrel.org/w/doku.php?id=banque_postale_renamer#la_banque_postale_et_des_pdfs). 
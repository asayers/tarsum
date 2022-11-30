# Tarsum

Like `tar -tf`, but it also includes a checksum of each file.

## Git integration

I wrote this because I have a git repo with some tarballs checked-in.
By default, `git diff` only tells you that a tarball has changed; you get
no indication of _what_ has changed.  Fortunately there's a mechanism for
extending `git diff` in cases like this.

Put this in your gitattributes:

```gitattributes
*.tar diff=tar
```

Put this in your gitconfig:

```ini
[diff "tar"]
    textconv = tarsum
```

The results?  Before:

```diff
diff --git a/tests/data/golden.tar b/tests/data/golden.tar
index 21e3343f2..f10ba66b4 100644
Binary files a/tests/data/golden.tar and b/tests/data/golden.tar differ
```

After:

```diff
diff --git a/tests/data/golden.tar b/tests/data/golden.tar
index 21e3343f2..f10ba66b4 100644
--- a/tests/data/golden.tar
+++ b/tests/data/golden.tar
@@ -1,5 +1,5 @@
 daily.json     341.6 KB 5db5480ddcfc0420ef71ea9b708dea982897ef0f
-k200.parquet    13.0 KB 718aaa447400094840558a099768b8d18b6b437a
+k200.parquet    13.1 KB dc20a5d669a5e35961120263ee7960e04ad13f81
 info.json        295 B  c69f2787761554480748ba872cf27feafaa94ffa
 other.parquet   13.8 KB 8a1e18eaee642f32b7979024eda50ddee2746e79
```

So, you still don't get to see what changed inside the files... but it's a
lot better than nothing!

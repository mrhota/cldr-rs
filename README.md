# cldr-rs
CLDR delivery and wrapper for Rust

# Versioning

Selecting a version, X.Y.Z, of this library will get you
the latest X.Y._ of CLDR. I need the freedom to produce
patch versions independent of CLDR's patches. This
explains why the first version of `cldr-rs` was version
29.0.0; to match the then-current version of CLDR.

I guess I'm more or less using semver because it seems like
people expect it, for better or worse.

# Caveats and Warnings

You should add this crate as a dependency at your own risk.
You should read the `build.rs` file and understand what it
does before building it.

So what does it do? Well, it downloads tons of CLDR data
from the unicode-cldr github repos, compresses it, and saves
it to Cargo's output directory. Then it deletes the JSON
data using `std::fs::remove_dir_all`, which I might have
gotten wrong. If you're not comfortable with that, don't
run the build script.

Anyway, you'll probably need about 150MB of free space to
store all the JSON + compressed data during build. After,
the compressed files are around 16MB.

# Unstable and Incomplete

Please be patient; the "library" isn't much of a library
yet. It basically only downloads CLDR data. Take the
"alpha" version tag seriously in the meantime.

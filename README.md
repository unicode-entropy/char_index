# char\_index
A crate for ~O(1) charwise indexing into a string, without normally using as much memory as a `Vec<char>`.

# Benchmarks
1000 "e"'s with 3 extra non ascii chars, shuffled:
|Implementation|Memory Use|Index 200th codepoint|
|-|-|-|
|`Vec<char>`|4,012B (+ 24B direct)|0.6ns|
|`IndexedChars`|2,009B (+ 64B direct)|4ns|
|`String`|1,006B (+ 24B direct)|126ns|  

(data collected using `benches/char_index.rs`)

# `no_std`
This crate is fully `no_std`, however it does rely on alloc.  
A std feature may be added at a later date, but it is currently unknown what that would include.

# License
This crate is licensed under MPL-2.0, this is a weak copyleft license intended to keep any modifications 
to the core library open source, without affecting other requirements greatly. This is not legal advice.

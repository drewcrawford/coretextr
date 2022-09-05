![logo](art/logo.png)

# Drew's fast Rust CoreText bindings

Provides select Rust bindings for Apple [Core Text](https://developer.apple.com/documentation/coretext?language=objc) framework.  This may be compared to the
[core-text](https://crates.io/crates/core-text) crate or some parts of the [core-foundation-rs](https://github.com/servo/core-foundation-rs) crate.

# Implementation status

* `CTFontDescriptor`
* `CTFontManager::CreateFontDescriptorFromData`
* Most of `SFNTLayoutTypes.h`

# Why this crate

Part of the [objr expanded universe](https://github.com/drewcrawford/objr#objr-expanded-universe), distinctive features of this library:

* Fast.  This crate is *significantly* faster than other approaches.  If you are interested in writing performance-critical realtime applications, this is the solution for you.
    * The full set of optimization is far too many to list, but the big idea is to either do what native ObjC/Swift applications do, or do something faster.
    * Compile-time selectors.  Most Rust crates do a runtime lookup for ObjC methods, which involves acquiring a lock and other yucky stuff, either on the first call or every call.  Instead, we do what real ObjC compilers do, which is way faster.  For more details, see [objr](https://github.com/drewcrawford/objr)
    * Smart pointers that provide global ARC inference.  Like ARC, you don't need to write manual retain/release calls.  Unlike ARC, the compiler
      usually doesn't need to write them either, meaning lower runtime memory management cost than even native code.  For more details, see [objr](https://github.com/drewcrawford/objr).
    * Runtime autorelease eliding, which keeps your objects out of autoreleasepools in common cases.  For more details, see [objr](https://github.com/drewcrawford/objr).
    * Pointer packing for optional types so they fit in a `u64`.  For more details, see [objr](https://github.com/drewcrawford/objr)
* Safe APIs.  Where possible APIs are designed with safe abstractions to provide familiar guarantees to Rust developers
* Low-level.  These bindings assume familiarity with bound APIs and are not documented separately.  For details on how they work, see the native documentation.
* Free for noncommercial or "small commercial" use.


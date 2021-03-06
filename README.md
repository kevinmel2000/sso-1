Web Single-sign-on framework
=============================

Single-sign-on framework written in Rust.

**Features**:

1. Open LDAP integration.
2. Embedded/Stand-alone session store backed by RocksDB.
3. Web UI interface for login and "soon" for access control management.
4. Restful API.

**Ingredients**:

* Open LDAP
* Nickel
* Semantic UI
* RocksDB

Development
-------------

**Prerequisites**:

1. Rust nightly.
2. Open LDAP.
3. RocksDB lib.
4. Inotify (for hot-reload/auto-compile).

For first init, please type:

    $ make init-dev

Run `./etc/script/devmon.sh` for hot reloading when template files or sources modified.

For compile only (no re-run) when source changes add `--compile-only` parameter:

    $ ./etc/script/devmon.sh --compile-only

By default `devmon.sh` using `example.toml` as the config file, if you have your own
you can set via env var, eg:

    $ CONFIG=dev.toml ./etc/script/devmon.sh

Troubleshooting
----------------

If you see error like this during build:

    error: file not found for module `build`

you need to type `make` first before `cargo build`.

If linker error like this:

    ld: library not found for -lrocksdb

you need to install rocksdb lib first, in OS X using Homebrew you can simply type:

    $ brew install rocksdb

API docs
-------------------

**/api/lookup**

Lookup access token for getting `uid`.

Parameters:

* `access_token` - access token gained from authorization step (login).

**/api/system/info**

For getting system information contains:

* server time.
* version.
* git revision.

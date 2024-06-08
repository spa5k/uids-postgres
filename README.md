Here's the updated README with the corrected links in the table:

# PostgreSQL Extension for Generating Unique IDs

<h1 align="center">
  uids
</h1>

```sql
postgres=# CREATE EXTENSION uids;
CREATE EXTENSION

postgres=# SELECT generate_typeid('user');
 generate_typeid
--------------------------------------
 user_01h2xcejqtf2nbrexx3vqjhp41
```

## Description

| Methodology               | Function                                    | Crate                                | Description                                              |
|---------------------------|---------------------------------------------|--------------------------------------|----------------------------------------------------------|
| [UUID v6][uuidv6]         | `generate_uuidv6()`                         | [`uuid`][crate-uuid]                 | UUID v6 ([RFC 4122][rfc-4122-update])                    |
|                           | `generate_uuidv6_text()`                    |                                      | UUID v6 as text                                          |
|                           | `generate_uuidv6_uuid()`                    |                                      | UUID v6 as Postgres UUID object                          |
| [UUID v7][uuidv7]         | `generate_uuidv7()`                         | [`uuid`][crate-uuid]                 | UUID v7 ([RFC 4122][rfc-4122-update])                    |
|                           | `generate_uuidv7_bytes()`                   |                                      | UUID v7 as bytes                                         |
|                           | `generate_uuidv7_from_string(TEXT)`         |                                      | Generate UUID v7 from a string                           |
|                           | `parse_uuidv7(TEXT)`                        |                                      | Parse UUID v7                                            |
| [NanoId][nanoid]          | `generate_nanoid()`                         | [`nanoid`][crate-nanoid]             | NanoID, developed by [Andrey Sitnik][github-ai]          |
|                           | `generate_nanoid_length(INT)`               |                                      | NanoID with a custom length                              |
|                           | `generate_nanoid_c(TEXT)`                   |                                      | NanoID with custom alphabets                             |
|                           | `generate_nanoid_length_c(INT, TEXT)`       |                                      | NanoID with custom length and alphabets                  |
| [Ksuid][ksuid]            | `generate_ksuid()`                          | [`svix-ksuid`][crate-svix-ksuid]     | Created by [Segment][segment]                            |
|                           | `generate_ksuid_bytes()`                    |                                      | KSUID as bytes                                           |
| [Ulid][ulid]              | `generate_ulid()`                           | [`ulid`][crate-ulid]                 | Unique, lexicographically sortable identifiers           |
|                           | `generate_ulid_bytes()`                     |                                      | ULID as bytes                                            |
|                           | `generate_ulid_from_string(TEXT)`           |                                      | Generate ULID from a string                              |
| [Timeflake][timeflake]    | `generate_timeflake()`                      | [`timeflake-rs`][crate-timeflake-rs] | Twitter's Snowflake + Instagram's ID + Firebase's PushID |
|                           | `generate_timeflake_bytes()`                |                                      | Timeflake as bytes                                       |
|                           | `generate_timeflake_uuid()`                 |                                      | Timeflake as UUID                                        |
| [PushId][pushid]          | `generate_pushid()`                         | [`pushid`][crate-pushid]             | Google Firebase's PushID                                 |
|                           | `generate_pushid_text()`                    |                                      | PushID as text                                           |
| [Cuid2][cuid2]            | `generate_cuid2()`                          | [`cuid2`][crate-cuid2]               | CUID2                                                    |
|                           | `check_cuid2(TEXT)`                         |                                      | Check if a string is a valid CUID2                       |
| [TypeId][typeid]          | `generate_typeid(TEXT)`                     | [`typeid`][crate-typeid]             | Generate TypeId with a specific prefix                   |
|                           | `check_typeid(TEXT, TEXT)`                  |                                      | Check if a TypeId matches a specific prefix              |

This Postgres extension is made possible thanks to [`pgrx`][pgrx].

## Supported IDs

1. [NanoId](https://github.com/ai/nanoid)
2. [Ksuid](https://github.com/segmentio/ksuid)
3. [Ulid](https://github.com/ulid/spec)
4. [TypeId](https://github.com/jetpack-io/typeid)
5. [UUIDv7](https://github.com/uuid-rs/uuid)
6. [Cuid2](https://github.com/paralleldrive/cuid2)
7. [PushId](https://github.com/firebase/firebase-js-sdk)
8. [Timeflake](https://github.com/anthonynsimon/timeflake)
9. [UUIDv6](https://github.com/uuid-rs/uuid)

[crate-uuid]: https://crates.io/crates/uuid
[crate-nanoid]: https://crates.io/crates/nanoid
[crate-svix-ksuid]: https://crates.io/crates/svix-ksuid
[crate-ulid]: https://crates.io/crates/ulid
[crate-timeflake-rs]: https://crates.io/crates/timeflake-rs
[crate-pushid]: https://crates.io/crates/pushid
[crate-cuid2]: https://crates.io/crates/cuid2
[crate-typeid]: https://crates.io/crates/typeid
[postgres]: https://www.postgresql.org/
[uuidv6]: https://github.com/uuid-rs/uuid
[uuidv7]: https://github.com/uuid-rs/uuid
[nanoid]: https://github.com/ai/nanoid
[ksuid]: https://github.com/segmentio/ksuid
[ulid]: https://github.com/ulid/spec
[timeflake]: https://github.com/anthonynsimon/timeflake
[pushid]: https://github.com/firebase/firebase-js-sdk
[cuid2]: https://github.com/paralleldrive/cuid2
[typeid]: https://github.com/jetpack-io/typeid
[pgrx]: https://github.com/zombodb/pgx



## Installation

### Non-Docker Environment

1. **Install Rust via rustup:**

    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```

2. **Prepare your PostgreSQL installation.**

3. **Install pgx:**

    ```bash
    cargo install cargo-pgx
    ```

4. **Initialize pgx for your PostgreSQL version:**

    ```bash
    cargo pgx init --pg14 $(which pg_config)
    ```

5. **Clone the repository and install the extension:**

    ```bash
    git clone https://github.com/spa5k/uids-postgres
    cd uids-postgres
    cargo pgx install
    ```

### Docker Environment

Refer to the included [Dockerfile](./docker/Dockerfile) for the installation template.

## Usage

### Enable the Extension

```sql
CREATE EXTENSION IF NOT EXISTS uids;
```

### Functions

#### KSUID

1. **Generate a new KSUID:**

    ```sql
    SELECT generate_ksuid();
    ```

    Example output:

    ```
    28KKKI8lpDkK2lHbAdWdgJYoLWF
    ```

2. **Generate KSUID bytes:**

    ```sql
    SELECT generate_ksuid_bytes();
    ```

    Example output:

    ```
    \x0ef557bc9b5b8027f222e2b32ed65e91b6bb8eb6
    ```

#### NanoId

1. **Generate a new NanoId (default size 21):**

    ```sql
    SELECT generate_nanoid();
    ```

    Example output:

    ```
    FfuwjZHjS5j5rATHVyl8M
    ```

2. **Generate a NanoId with a custom size:**

    ```sql
    SELECT generate_nanoid_length(10);
    ```

    Example output:

    ```
    V2D2D7-dnw
    ```

3. **Generate a NanoId with custom alphabets (length 21):**

    ```sql
    SELECT generate_nanoid_c('1234567890abcdef');
    ```

    Example output:

    ```
    6df80ad84587f4a20838c
    ```

4. **Generate a NanoId with custom alphabets and custom length:**

    ```sql
    SELECT generate_nanoid_length_c(10, '1234567890abcdef');
    ```

    Example output:

    ```
    050487bff0
    ```

#### Ulid

1. **Generate a new Ulid:**

    ```sql
    SELECT generate_ulid();
    ```

    Example output:

    ```
    01G1JE4GXWC1A9PXHG0SXQDE1J
    ```

2. **Generate Ulid bytes:**

    ```sql
    SELECT generate_ulid_bytes();
    ```

    Example output:

    ```
    \x018064e2bff9e6bb876aa8948e50d9c6
    ```

3. **Generate Ulid from a custom string:**

    ```sql
    SELECT generate_ulid_from_string('01CAT3X5Y5G9A62F1rFA6Tnice');
    ```

    Example output:

    ```
    01CAT3X5Y5G9A62F1RFA6TN1CE
    ```

#### TypeId

1. **Generate a TypeId with a specific prefix:**

    ```sql
    SELECT generate_typeid('user');
    ```

    Example output:

    ```
    user_01h2xcejqtf2nbrexx3vqjhp41
    ```

2. **Check if a TypeId matches a specific prefix:**

    ```sql
    SELECT check_typeid('user', 'user_01h2xcejqtf2nbrexx3vqjhp41');
    ```

    Example output:

    ```
    true
    ```

#### UUIDv7

1. **Generate a new UUIDv7:**

    ```sql
    SELECT generate_uuidv7();
    ```

    Example output:

    ```
    01809424-3e59-7c05-9219-566f82fff672
    ```

2. **Generate UUIDv7 bytes:**

    ```sql
    SELECT generate_uuidv7_bytes();
    ```

    Example output:

    ```
    \x018094243e597c059219566f82fff672
    ```

3. **Generate UUIDv7 from a string:**

    ```sql
    SELECT generate_uuidv7_from_string('67e55044-10b1-426f-9247-bb680e5fe0c8');
    ```

    Example output:

    ```
    67e55044-10b1-426f-9247-bb680e5fe0c8
    ```

4. **Parse UUIDv7:**

    ```sql
    SELECT parse_uuidv7('67e55044-10b1-426f-9247-bb680e5fe0c8');
    ```

    Example output:

    ```
    67e55044-10b1-426f-9247-bb680e5fe0c8
    ```

#### Cuid2

1. **Generate a new Cuid2:**

    ```sql
    SELECT generate_cuid2();
    ```

    Example output:

    ```
    cl8f8y8f80000000000000000
    ```

2. **Check if a string is a valid Cuid2:**

    ```sql
    SELECT check_cuid2('cl8f8y8f80000000000000000');
    ```

    Example output:

    ```
    true
    ```

#### PushId

1. **Generate a new PushId:**

    ```sql
    SELECT generate_pushid();
    ```

    Example output:

    ```
    -MZ1e2f3g4h5i6j7k8l9
    ```

2. **Generate a PushId as text:**

    ```sql
    SELECT generate_pushid_text();
    ```

    Example output:

    ```
    -MZ1e2f3g4h5i6j7k8l9
    ```

#### Timeflake

1. **Generate a new Timeflake:**

    ```sql
    SELECT generate_timeflake();
    ```

    Example output:

    ```
    01F8MECHJ8KZ9Q9J8KZ9Q9J8KZ
    ```

2. **Generate Timeflake bytes:**

    ```sql
    SELECT generate_timeflake_bytes();
    ```

    Example output:

    ```
    \x018064e2bff9e6bb876aa8948e50d9c6
    ```

3. **Generate Timeflake UUID:**

    ```sql
    SELECT generate_timeflake_uuid();
    ```

    Example output:

    ```
    018064e2-bff9-e6bb-876a-a8948e50d9c6
    ```

#### UUIDv6

1. **Generate a new UUIDv6:**

    ```sql
    SELECT generate_uuidv6();
    ```

    Example output:

    ```
    1e4eaa4e-7c4b-6e5d-8a4e-7c4b6e5d8a4e
    ```

2. **Generate a UUIDv6 as text:**

    ```sql
    SELECT generate_uuidv6_text();
    ```

    Example output:

    ```
    1e4eaa4e-7c4b-6e5d-8a4e-7c4b6e5d8a4e
    ```

3. **Generate a UUIDv6 as a Postgres UUID object:**

    ```sql
    SELECT generate_uuidv6_uuid();
    ```

    Example output:

    ```
    1e4eaa4e-7c4b-6e5d-8a4e-7c4b6e5d8a4e
    ```

This setup provides a comprehensive set of functions to generate and validate various types of unique identifiers within a PostgreSQL extension using `pgx`.

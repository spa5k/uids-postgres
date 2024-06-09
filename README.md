# PostgreSQL Extension for Generating Unique IDs

<h1 align="center">
  uids-postgres
</h1>

```sql
postgres=# CREATE EXTENSION uids;
CREATE EXTENSION

postgres=# SELECT generate_typeid('user');
 generate_typeid
--------------------------------------
 user_01h2xcejqtf2nbrexx3vqjhp41
```
## Why should I use this?

Using the `uids-postgres` extension for generating unique IDs in PostgreSQL offers several advantages:

### 1. **Diverse ID Generation Methods**
The extension supports multiple methodologies for generating unique IDs, including UUID v6, UUID v7, NanoId, Ksuid, Ulid, Timeflake, PushId, and Cuid2. This flexibility allows you to choose the most suitable ID generation strategy for your specific use case, whether you need time-based IDs, lexicographically sortable IDs, or IDs with custom prefixes.

### 2. **Enhanced Data Integrity and Uniqueness**
Using unique identifiers like UUIDs ensures data integrity and uniqueness across distributed systems. This is particularly important for applications that require federated data or need to avoid collisions in a multi-node environment.

### 3. **Performance and Efficiency**
Generating unique IDs within the database can be more efficient than handling this in the application layer. This reduces the overhead of network communication and ensures that ID generation is consistent and centralized.

### 4. **Ease of Use**
The extension provides simple SQL functions to generate and validate various types of unique IDs. This makes it easy to integrate into existing SQL workflows without requiring significant changes to your application code.

### 5. **Compliance and Security**
For applications that require compliance with data security standards, using IDs like UUIDs can help meet these requirements. The extension ensures that IDs are generated in a secure manner, reducing the risk of predictable or duplicate IDs.

### 6. **Community and Ecosystem**
PostgreSQL has a rich ecosystem of extensions that enhance its capabilities. By using `uids-postgres`, you leverage the power of PostgreSQL's extensibility, allowing you to tailor your database to meet specific needs without altering its core architecture.

### 7. **Scalability**
Unique IDs like UUIDs and Ksuid are designed to be scalable and can handle large volumes of data efficiently. This is crucial for applications that need to maintain good performance as they grow to handle billions of rows.

### 8. **Compatibility with Modern Applications**
Many modern applications, especially those involving microservices, distributed systems, and IoT, benefit from using unique identifiers. The `uids-postgres` extension supports various ID types that are well-suited for these environments, ensuring compatibility and ease of integration.

By using the `uids-postgres` extension, you can enhance your PostgreSQL database with robust, flexible, and efficient unique ID generation capabilities, making it a valuable tool for a wide range of applications.

## Why should I use ulid/typeid/other ids etc over uuid?


Choosing ULID or TypeID/ etc over UUID can offer several advantages depending on your specific use case. Here are some key reasons:

### Advantages of ULID over UUID

1. **Lexicographical Sortability**
   - ULIDs are designed to be lexicographically sortable, which means they can be sorted in alphabetical order. This is particularly useful for databases and systems that need to quickly sort and search large numbers of identifiers.

2. **Faster Generation**
   - ULIDs use a cryptographically secure pseudorandom number generator (CSPRNG) for the random component, which is faster than the method used for generating UUIDs. Benchmarks have shown that ULID generation can be up to 50% faster than UUID generation, making them suitable for high-volume environments.

3. **Compactness and URL-Safety**
   - ULIDs are more compact, requiring only 26 characters compared to UUIDs' 36 characters. They are also URL-safe, meaning they can be used in URLs without the need for encoding or escaping, which is beneficial for web applications and APIs.

4. **Timestamp Encoding**
   - ULIDs include a timestamp component, which allows them to be sorted by creation order. This feature is useful for tracking the order of events in distributed systems and for data partitioning and indexing in NoSQL databases.

### Advantages of TypeID over UUID

1. **Custom Prefixes**
   - TypeIDs allow you to generate unique identifiers with specific prefixes, which can be useful for categorizing and managing different types of entities within your system. This feature is not available with standard UUIDs.

2. **Readability and Context**
   - The custom prefix in TypeIDs can provide additional context about the type of entity the ID represents, making it easier to understand and manage the data.

### When to Use UUID

1. **Cryptographic Security**
   - If your application requires identifiers that are truly random and have no predictable pattern, such as for cryptographic or security purposes, UUIDs are a better choice. ULIDs, by design, are not intended to be cryptographically secure and should not be used for sensitive applications.

2. **Standardization**
   - UUIDs are standardized by RFC 4122, which means they are widely supported and recognized across different systems and platforms. This can be important when working with vendors or integrating with third-party systems.

### Summary

- **Use ULID** if you need lexicographically sortable identifiers, faster generation speeds, compactness, and URL-safety.
- **Use TypeID** if you need custom prefixes for better readability and context.
- **Use UUID** if you need cryptographic security, standardization, and do not require sorting capabilities.

Choosing between these options depends on the specific requirements of your application, such as the need for sorting, speed of generation, security, and readability.



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
[pgrx]: https://github.com/pgcentralfoundation/pgrx



## Installation

Use [pgrx](https://github.com/pgcentralfoundation/pgrx). You can clone this repo and install this extension locally by following [this guide](https://github.com/pgcentralfoundation/pgrx/blob/develop/cargo-pgrx/README.md#installing-your-extension-locally).

You can also download relevant files from [releases](https://github.com/spa5k/uids-postgres/releases) page.

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



### References
- [Timescale Blog: Top 8 PostgreSQL Extensions](https://www.timescale.com/blog/top-8-postgresql-extensions/)
- [Stack Overflow: What is uuid-ossp in Postgres](https://stackoverflow.com/questions/21709800/what-is-uuid-ossp-in-postgres)
- [Dev.to: Exploring PostgreSQL Extensions](https://dev.to/nilelazarus/exploring-postgresql-extensions-enhance-your-database-capabilities-576o)
- [Medium: UUID vs ULID, How ULID improves write speeds](https://medium.com/@sammaingi5/uuid-vs-ulid-how-ulid-improves-write-speeds-d16b23505458)
- [Solwey: IDs: Integer Vs UUID Vs ULID](https://www.solwey.com/posts/ids-integer-vs-uuid-vs-ulid)
- https://github.com/fboulnois/pg_uuidv7
- https://www.postgresql.org/docs/current/uuid-ossp.html
- https://dba.stackexchange.com/questions/102448/how-should-i-index-a-uuid-in-postgres
- https://pganalyze.com/blog/5mins-postgres-uuid-vs-serial-primary-keys
- https://news.ycombinator.com/item?id=36429986
- https://www.postgresql.org/docs/current/extend-extensions.html
- https://medium.com/@sammaingi5/uuid-vs-ulid-how-ulid-improves-write-speeds-d16b23505458
- https://www.solwey.com/posts/ids-integer-vs-uuid-vs-ulid
- https://news.ycombinator.com/item?id=40016413
- https://www.reddit.com/r/programming/comments/1ckklm9/why_choose_ulids_over_traditional_uuids_or_ids/
- https://www.linkedin.com/pulse/should-i-use-uuid-ulid-unique-ids-systems-pablo-puma-ihdoc
- https://dev.to/0xfedev/the-uniquity-chronicles-exploring-the-cosmos-of-unique-id-algorithms-31d6
- https://blog.hassam.dev/ulid-uuid-integer-ids/
- https://github.com/jetify-com/typeid
- https://stackoverflow.com/questions/59919519/mysql-is-an-unique-index-necessary-on-a-uuid-column

> In recent additions to this project, the new ids, and the dockerfile were taken from [pgx_ulid](https://github.com/pksunkara/pgx_ulid), alongside some help from  [pg_idkit](https://github.com/VADOSWARE/pg_idkit).
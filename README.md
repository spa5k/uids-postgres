# PostgreSQL Extension to generate various type of Unique IDS.

## 1. Supported IDs

1. [NanoId](https://github.com/ai/nanoid)

2. [Ksuid](https://github.com/segmentio/ksuid)

3. [Ulid](https://github.com/ulid/spec)

## 2. Installation

### 2.1. Installation on non docker environment

2.1.1. Install rust through rustup.

```bash
curl https://sh.rustup.rs -sSf | sh
```

2.1.2. Prepare your postgres installation
2.1.3. Install pgx

```bash
cargo install cargo-pgx
```

2.1.4. Initialize pgx for the postgres version you have already installed

Handle the number accordingly.

```bash
cargo pgx init --pg14 $(which pg_config)
```

2.1.5. Install the extension

```bash
git clone https://github.com/spa5k/uids-postgres \
&& cd uids-postgres \
&& cargo pgx install
```

### 2.2 Installation on docker environment

Check the included [Dockerfile](./docker/Dockerfile) for the installation template.

## 3. Functions available

### 3.0. Enable the extension

```sql
CREATE EXTENSION IF NOT EXISTS uids;
```

### 3.1. KSUID -

1. Generate a new KSUID

```sql
select generate_ksuid();

-----------------------------

28KKKI8lpDkK2lHbAdWdgJYoLWF
```

2. Generate a KSUID bytes.

```sql
select generate_ksuid_bytes();

-----------------------------

\x0ef557bc9b5b8027f222e2b32ed65e91b6bb8eb6
```

### 3.2. NanoId -

1. Generate a new NanoId with default size of 21

```sql
select generate_nanoid();

-----------------------------

FfuwjZHjS5j5rATHVyl8M
```

2. Generate a NanoId with a custom size

```sql
select generate_nanoid_length(10);

-----------------------------

V2D2D7-dnw
```

3. Generate a NanoId with a custom alphabets with length of 21

```sql
-- Length of the nanoid is first argument, while the alphabets one is second.
select generate_nanoid_c('1234567890abcdef');

-----------------------------

6df80ad84587f4a20838c
```

4. Generate a NanoId with a custom alphabets and custom length

```sql
-- Length of the nanoid is first argument, while the alphabets one is second.
select generate_nanoid_length_c(10, '1234567890abcdef');

-----------------------------

050487bff0
```

### 3.3. Ulid -

1. Generate a new Ulid

```sql
select generate_ulid();

-----------------------------

01G1JE4GXWC1A9PXHG0SXQDE1J
```

2. Generate Ulid bytes

```sql
select generate_ulid_bytes();

-----------------------------

\x018064e2bff9e6bb876aa8948e50d9c6
```

3. Generate Ulid from a custom string

```sql
select generate_ulid_from_string('01CAT3X5Y5G9A62F1rFA6Tnice');

-----------------------------

01CAT3X5Y5G9A62F1RFA6TN1CE
```

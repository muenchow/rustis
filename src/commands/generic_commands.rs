use crate::{
    cmd,
    resp::{BulkString, FromSingleValueArray, FromValue, Value},
    Command, CommandSend, Error, Future, SingleArgOrCollection,
};

/// A group of generic Redis commands
///
/// # See Also
/// [Redis Generic Commands](https://redis.io/commands/?group=generic)
pub trait GenericCommands: CommandSend {
    /// This command copies the value stored at the source key to the destination key.
    ///
    /// # See Also
    /// [https://redis.io/commands/copy/](https://redis.io/commands/copy/)
    fn copy<S, D>(&self, source: S, destination: D) -> Copy<Self>
    where
        S: Into<BulkString>,
        D: Into<BulkString>,
    {
        Copy {
            generic_commands: &self,
            cmd: cmd("COPY").arg(source).arg(destination),
        }
    }

    /// Removes the specified keys. A key is ignored if it does not exist.
    ///
    /// # Return
    /// The number of keys that were removed.
    ///
    /// # See Also
    /// [https://redis.io/commands/del/](https://redis.io/commands/del/)
    fn del<K, C>(&self, keys: C) -> Future<'_, usize>
    where
        K: Into<BulkString>,
        C: SingleArgOrCollection<K>,
    {
        self.send_into(cmd("DEL").arg(keys))
    }

    /// Serialize the value stored at key in a Redis-specific format and return it to the user.
    ///
    /// # Return
    /// The serialized value.
    ///
    /// # See Also
    /// [https://redis.io/commands/dump/](https://redis.io/commands/dump/)
    fn dump<K>(&self, key: K) -> Future<'_, Vec<u8>>
    where
        K: Into<BulkString>,
    {
        let fut = self.send_into::<Value>(cmd("DUMP").arg(key));
        Box::pin(async move {
            let value = fut.await?;
            match value {
                Value::BulkString(BulkString::Binary(b)) => Ok(b),
                _ => Err(Error::Internal("Unexpected dump format".to_owned())),
            }
        })
    }

    /// Returns if keys exist.
    ///
    /// # Return
    /// The number of keys that exist from those specified as arguments.
    ///
    /// # See Also
    /// [https://redis.io/commands/exists/](https://redis.io/commands/exists/)
    fn exists<K, C>(&self, keys: C) -> Future<'_, usize>
    where
        K: Into<BulkString>,
        C: SingleArgOrCollection<K>,
    {
        self.send_into(cmd("EXISTS").arg(keys))
    }

    /// Set a timeout on key in seconds
    ///
    /// # Return
    /// * `true` - if the timeout was set.
    /// * `false` - if the timeout was not set. e.g. key doesn't exist, or operation skipped due to the provided arguments.
    ///
    /// # See Also
    /// [https://redis.io/commands/expire/](https://redis.io/commands/expire/)
    fn expire<K>(&self, key: K, seconds: u64) -> Expire<Self>
    where
        K: Into<BulkString>,
    {
        Expire {
            generic_commands: &self,
            cmd: cmd("EXPIRE").arg(key).arg(seconds),
        }
    }

    /// EXPIREAT has the same effect and semantic as EXPIRE,
    /// but instead of specifying the number of seconds representing the TTL (time to live),
    /// it takes an absolute Unix timestamp (seconds since January 1, 1970)
    ///
    /// A timestamp in the past will delete the key
    ///
    /// # Return
    /// * `true` - if the timeout was set.
    /// * `false` - if the timeout was not set. e.g. key doesn't exist, or operation skipped due to the provided arguments.
    ///
    /// # See Also
    /// [https://redis.io/commands/expireat/](https://redis.io/commands/expireat/)
    fn expireat<K>(&self, key: K, unix_time_seconds: u64) -> Expire<Self>
    where
        K: Into<BulkString>,
    {
        Expire {
            generic_commands: &self,
            cmd: cmd("EXPIREAT").arg(key).arg(unix_time_seconds),
        }
    }

    /// Returns the absolute Unix timestamp (since January 1, 1970) in seconds at which the given key will expire.
    ///
    /// # Return
    /// Expiration Unix timestamp in seconds, or a negative value in order to signal an error (see the description below).
    /// - The command returns -1 if the key exists but has no associated expiration time.
    /// - The command returns -2 if the key does not exist.
    ///
    /// # See Also
    /// [https://redis.io/commands/expiretime/](https://redis.io/commands/expiretime/)
    fn expiretime<K>(&self, key: K) -> Future<'_, i64>
    where
        K: Into<BulkString>,
    {
        self.send_into(cmd("EXPIRETIME").arg(key))
    }

    /// Returns all keys matching pattern.
    ///
    /// # Return
    /// list of keys matching pattern.
    ///
    /// # See Also
    /// [https://redis.io/commands/keys/](https://redis.io/commands/keys/)
    fn keys<P, K, A>(&self, pattern: P) -> Future<'_, A>
    where
        P: Into<BulkString>,
        K: FromValue,
        A: FromSingleValueArray<K>,
    {
        self.send_into(cmd("KEYS").arg(pattern))
    }

    /// Move key from the currently selected database to the specified destination database.
    ///
    /// # Return
    /// * `true` - if key was moved.
    /// * `false` - f key was not moved.
    ///
    /// # See Also
    /// [https://redis.io/commands/move/](https://redis.io/commands/move/)
    fn move_<K>(&self, key: K, db: usize) -> Future<'_, i64>
    where
        K: Into<BulkString>,
    {
        self.send_into(cmd("MOVE").arg(key).arg(db))
    }

    /// Returns the internal encoding for the Redis object stored at `key`
    ///
    /// # Return
    /// The encoding of the object, or nil if the key doesn't exist
    ///
    /// # See Also
    /// [https://redis.io/commands/object-encoding/](https://redis.io/commands/object-encoding/)
    fn object_encoding<K, E>(&self, key: K) -> Future<'_, E>
    where
        K: Into<BulkString>,
        E: FromValue,
    {
        self.send_into(cmd("OBJECT").arg("ENCODING").arg(key))
    }

    /// This command returns the logarithmic access frequency counter of a Redis object stored at `key`.
    ///
    /// # Return
    /// The counter's value.
    ///
    /// # See Also
    /// [https://redis.io/commands/object-freq/](https://redis.io/commands/object-freq/)
    fn object_freq<K>(&self, key: K) -> Future<'_, i64>
    where
        K: Into<BulkString>,
    {
        self.send_into(cmd("OBJECT").arg("FREQ").arg(key))
    }

    /// This command returns the time in seconds since the last access to the value stored at `key`.
    ///
    /// # Return
    /// The idle time in seconds.
    ///
    /// # See Also
    /// [https://redis.io/commands/object-idletime/](https://redis.io/commands/object-idletime/)
    fn object_idle_time<K>(&self, key: K) -> Future<'_, i64>
    where
        K: Into<BulkString>,
    {
        self.send_into(cmd("OBJECT").arg("IDLETIME").arg(key))
    }

    /// This command returns the reference count of the stored at `key`.
    ///
    /// # Return
    /// The number of references.
    ///
    /// # See Also
    /// [https://redis.io/commands/object-refcount/](https://redis.io/commands/object-refcount/)
    fn object_refcount<K>(&self, key: K) -> Future<'_, i64>
    where
        K: Into<BulkString>,
    {
        self.send_into(cmd("OBJECT").arg("REFCOUNT").arg(key))
    }

    /// Remove the existing timeout on key,
    /// turning the key from volatile (a key with an expire set)
    /// to persistent (a key that will never expire as no timeout is associated).
    ///
    /// # Return
    /// * `true` - if the timeout was removed.
    /// * `false` - if key does not exist or does not have an associated timeout.
    ///
    /// # See Also
    /// [https://redis.io/commands/persist/](https://redis.io/commands/persist/)
    fn persist<K>(&self, key: K) -> Future<'_, bool>
    where
        K: Into<BulkString>,
    {
        self.send_into(cmd("PERSIST").arg(key))
    }

    /// This command works exactly like EXPIRE but the time to live of the key is specified in milliseconds instead of seconds.
    ///
    /// # Return
    /// * `true` - if the timeout was set.
    /// * `false` - if the timeout was not set. e.g. key doesn't exist, or operation skipped due to the provided arguments.
    ///
    /// # See Also
    /// [https://redis.io/commands/pexpire/](https://redis.io/commands/pexpire/)
    fn pexpire<K>(&self, key: K, milliseconds: u64) -> Expire<Self>
    where
        K: Into<BulkString>,
    {
        Expire {
            generic_commands: &self,
            cmd: cmd("PEXPIRE").arg(key).arg(milliseconds),
        }
    }

    /// PEXPIREAT has the same effect and semantic as EXPIREAT,
    /// but the Unix time at which the key will expire is specified in milliseconds instead of seconds.
    ///
    /// # Return
    /// * `true` - if the timeout was set.
    /// * `false` - if the timeout was not set. e.g. key doesn't exist, or operation skipped due to the provided arguments.
    ///
    /// # See Also
    /// [https://redis.io/commands/pexpireat/](https://redis.io/commands/pexpireat/)
    fn pexpireat<K>(&self, key: K, unix_time_milliseconds: u64) -> Expire<Self>
    where
        K: Into<BulkString>,
    {
        Expire {
            generic_commands: &self,
            cmd: cmd("PEXPIREAT").arg(key).arg(unix_time_milliseconds),
        }
    }

    /// PEXPIRETIME has the same semantic as EXPIRETIME,
    /// but returns the absolute Unix expiration timestamp in milliseconds instead of seconds.
    ///
    /// # Return
    ///  Expiration Unix timestamp in milliseconds, or a negative value in order to signal an error (see the description below).
    /// - The command returns -1 if the key exists but has no associated expiration time.
    /// - The command returns -2 if the key does not exist.
    ///
    /// # See Also
    /// [https://redis.io/commands/pexpiretime/](https://redis.io/commands/pexpiretime/)
    fn pexpiretime<K>(&self, key: K) -> Future<'_, i64>
    where
        K: Into<BulkString>,
    {
        self.send_into(cmd("PEXPIRETIME").arg(key))
    }

    /// Returns the remaining time to live of a key that has a timeout.
    ///
    /// # Return
    /// TTL in milliseconds, or a negative value in order to signal an error:
    /// -2 if the key does not exist.
    /// -1 if the key exists but has no associated expire.
    ///
    /// # See Also
    /// [https://redis.io/commands/pttl/](https://redis.io/commands/pttl/)
    fn pttl<K>(&self, key: K) -> Future<'_, i64>
    where
        K: Into<BulkString>,
    {
        self.send_into(cmd("PTTL").arg(key))
    }

    /// Return a random key from the currently selected database.
    ///
    /// # Return
    /// The number of references.
    ///
    /// # See Also
    /// [https://redis.io/commands/randomkey/](https://redis.io/commands/randomkey/)
    fn randomkey<R>(&self) -> Future<'_, R>
    where
        R: FromValue,
    {
        self.send_into(cmd("RANDOMKEY"))
    }

    /// Renames key to newkey.
    ///
    /// # See Also
    /// [https://redis.io/commands/rename/](https://redis.io/commands/rename/)
    fn rename<K1, K2>(&self, key: K1, new_key: K2) -> Future<'_, ()>
    where
        K1: Into<BulkString>,
        K2: Into<BulkString>,
    {
        self.send_into(cmd("RENAME").arg(key).arg(new_key))
    }

    /// Renames key to newkey if newkey does not yet exist. 
    /// It returns an error when key does not exist.
    /// 
    /// # Return
    /// * `true` if key was renamed to newkey.
    /// * `false` if newkey already exists.
    /// # See Also
    /// [https://redis.io/commands/renamenx/](https://redis.io/commands/renamenx/)
    fn renamenx<K1, K2>(&self, key: K1, new_key: K2) -> Future<'_, bool>
    where
        K1: Into<BulkString>,
        K2: Into<BulkString>,
    {
        self.send_into(cmd("RENAMENX").arg(key).arg(new_key))
    }

    /// Iterates the set of keys in the currently selected Redis database.
    /// 
    /// # Return
    /// A list of keys
    /// 
    /// # See Also
    /// [https://redis.io/commands/scan/](https://redis.io/commands/scan/)
    fn scan(&self, cursor: u64) -> Scan<Self>
    {
        Scan {
            generic_commands: &self,
            cmd: cmd("SCAN").arg(cursor)
        }
    }

    /// Returns the remaining time to live of a key that has a timeout.
    ///
    /// # Return
    /// TTL in seconds, or a negative value in order to signal an error:
    /// -2 if the key does not exist.
    /// -1 if the key exists but has no associated expire.
    ///
    /// # See Also
    /// [https://redis.io/commands/ttl/](https://redis.io/commands/ttl/)
    fn ttl<K>(&self, key: K) -> Future<'_, i64>
    where
        K: Into<BulkString>,
    {
        self.send_into(cmd("TTL").arg(key))
    }

    /// Create a key associated with a value that is obtained by deserializing
    /// the provided serialized value (obtained via DUMP).
    ///
    /// # Return
    /// Restore command builder
    ///
    /// # See Also
    /// [https://redis.io/commands/restore/](https://redis.io/commands/restore/)
    fn restore<K>(&self, key: K, ttl: u64, serialized_value: Vec<u8>) -> Restore<Self>
    where
        K: Into<BulkString>,
    {
        Restore {
            generic_commands: &self,
            cmd: cmd("RESTORE")
                .arg(key)
                .arg(ttl)
                .arg(BulkString::Binary(serialized_value)),
        }
    }

    /// Returns the string representation of the type of the value stored at key.
    ///
    /// The different types that can be returned are: string, list, set, zset, hash and stream.
    ///
    /// # Return
    /// type of key, or empty string when key does not exist.
    ///
    /// # See Also
    /// [https://redis.io/commands/type/](https://redis.io/commands/type/)
    fn type_<K>(&self, key: K) -> Future<'_, String>
    where
        K: Into<BulkString>,
    {
        self.send_into(cmd("TYPE").arg(key))
    }

    /// This command is very similar to DEL: it removes the specified keys. 
    ///
    /// # Return
    /// The number of keys that were unlinked.
    ///
    /// # See Also
    /// [https://redis.io/commands/unlink/](https://redis.io/commands/unlink/)
    fn unlink<K, C>(&self, keys: C) -> Future<'_, usize>
    where
        K: Into<BulkString>,
        C: SingleArgOrCollection<K>,
    {
        self.send_into(cmd("UNLINK").arg(keys))
    }
}

/// Builder for the [copy](crate::GenericCommands::copy) command
pub struct Copy<'a, T: GenericCommands + ?Sized> {
    generic_commands: &'a T,
    cmd: Command,
}

impl<'a, T: GenericCommands> Copy<'a, T> {
    /// Allows specifying an alternative logical database index for the destination key.
    pub fn db(self, destination_db: usize) -> Self {
        Self {
            generic_commands: self.generic_commands,
            cmd: self.cmd.arg("DB").arg(destination_db),
        }
    }

    /// Removes the destination key before copying the value to it
    pub fn replace(self) -> Self {
        Self {
            generic_commands: self.generic_commands,
            cmd: self.cmd.arg("REPLACE"),
        }
    }

    /// Execute the command
    ///
    /// # Return
    ///  Success of the operation
    pub fn execute(self) -> Future<'a, bool> {
        self.generic_commands.send_into(self.cmd)
    }
}

/// Builder for the [expire](crate::GenericCommands::expire) command
pub struct Expire<'a, T: GenericCommands + ?Sized> {
    generic_commands: &'a T,
    cmd: Command,
}

impl<'a, T: GenericCommands> Expire<'a, T> {
    /// Set expiry only when the key has no expiry
    pub fn nx(self) -> Future<'a, bool> {
        self.generic_commands.send_into(self.cmd.arg("NX"))
    }

    /// Set expiry only when the key has an existing expiry
    pub fn xx(self) -> Future<'a, bool> {
        self.generic_commands.send_into(self.cmd.arg("XX"))
    }

    /// Set expiry only when the new expiry is greater than current one
    pub fn gt(self) -> Future<'a, bool> {
        self.generic_commands.send_into(self.cmd.arg("GT"))
    }

    /// Set expiry only when the new expiry is less than current one
    pub fn lt(self) -> Future<'a, bool> {
        self.generic_commands.send_into(self.cmd.arg("LT"))
    }

    /// execute with no option
    pub fn execute(self) -> Future<'a, bool> {
        self.generic_commands.send_into(self.cmd)
    }
}

/// Builder for the [restore](crate::GenericCommands::restore) command
pub struct Restore<'a, T: GenericCommands + ?Sized> {
    generic_commands: &'a T,
    cmd: Command,
}

impl<'a, T: GenericCommands> Restore<'a, T> {
    /// Force replacing the key if it already exists
    pub fn replace(self) -> Self {
        Self {
            generic_commands: self.generic_commands,
            cmd: self.cmd.arg("REPLACE"),
        }
    }

    /// If the ABSTTL modifier was used, ttl should represent
    /// an absolute Unix timestamp (in milliseconds) in which the key will expire.
    pub fn abs_ttl(self) -> Self {
        Self {
            generic_commands: self.generic_commands,
            cmd: self.cmd.arg("ABSTTL"),
        }
    }

    /// For eviction purposes, you may use the IDLETIME or FREQ modifiers.
    pub fn idle_time(self, seconds: i64) -> Self {
        Self {
            generic_commands: self.generic_commands,
            cmd: self.cmd.arg("IDLETIME").arg(seconds),
        }
    }

    /// For eviction purposes, you may use the IDLETIME or FREQ modifiers.
    pub fn freq(self, frequency: f64) -> Self {
        Self {
            generic_commands: self.generic_commands,
            cmd: self.cmd.arg("FREQ").arg(frequency),
        }
    }

    /// Execute the command
    pub fn execute(self) -> Future<'a, ()> {
        self.generic_commands.send_into(self.cmd)
    }
}

/// Builder for the [scan](crate::GenericCommands::scan) command
pub struct Scan<'a, T: GenericCommands + ?Sized> {
    generic_commands: &'a T,
    cmd: Command,
}

impl<'a, T: GenericCommands> Scan<'a, T> {
    pub fn match_<P>(self, pattern: P) -> Self
    where
        P: Into<BulkString>,
    {
        Self {
            generic_commands: self.generic_commands,
            cmd: self.cmd.arg("MATCH").arg(pattern),
        }
    }

    pub fn count(self, count: usize) -> Self {
        Self {
            generic_commands: self.generic_commands,
            cmd: self.cmd.arg("COUNT").arg(count),
        }
    }

    /// You can use the TYPE option to ask SCAN to only return objects that match a given type
    pub fn type_<A>(self, type_: A) -> Self 
    where 
        A : Into<BulkString>
    {
        Self {
            generic_commands: self.generic_commands,
            cmd: self.cmd.arg("TYPE").arg(type_),
        }  
    }

    /// Execute the command
    pub fn execute<K, A>(self) -> Future<'a, (u64, A)>
    where
        K: FromValue,
        A: FromSingleValueArray<K> + Default
    {
        self.generic_commands.send_into(self.cmd)
    }
}

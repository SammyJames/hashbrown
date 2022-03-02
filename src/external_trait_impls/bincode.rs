mod map {
    use bincode::{Encode, Decode};
    use bincode::de::Decoder;
    use bincode::enc::Encoder;
    use bincode::error::{DecodeError, EncodeError};
    use core::hash::{BuildHasher, Hash};
    use crate::hash_map::HashMap;

    impl<K, V, H> Encode for HashMap<K, V, H>
    where
        K: Encode + Eq + Hash,
        V: Encode,
        H: BuildHasher,
    {
        #[cfg_attr(feature = "inline-more", inline)]
        fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
            let len = self.len();
            len.encode(encoder)?;
            for (k, v) in self.iter() {
                bincode::Encode::encode(k, encoder)?;
                bincode::Encode::encode(v, encoder)?;
            }
            Ok(())
        }
    }

    impl<K, V, H> Decode for HashMap<K, V, H>
    where
        K: Decode + Eq + Hash,
        V: Decode,
        H: BuildHasher + Default,
    {
        #[cfg_attr(feature = "inline-more", inline)]
        fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
            let len = usize::decode(decoder)?;
            decoder.claim_container_read::<(K, V)>(len)?;

            let mut result = HashMap::with_capacity_and_hasher(len, H::default());
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<(K, V)>());
                let k = K::decode(decoder)?;
                let v = V::decode(decoder)?;
                result.insert(k, v);
            }
            Ok(result)
        }
    }
}

mod set {
    use bincode::{Encode, Decode};
    use bincode::de::Decoder;
    use bincode::enc::Encoder;
    use bincode::error::{DecodeError, EncodeError};
    use core::hash::{BuildHasher, Hash};
    use crate::hash_set::HashSet;

    impl<K, H> Encode for HashSet<K, H>
    where
        K: Encode + Eq + Hash,
        H: BuildHasher,
    {
        #[cfg_attr(feature = "inline-more", inline)]
        fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
            let len = self.len();
            len.encode(encoder)?;
            for entry in self.iter() {
                bincode::Encode::encode(entry, encoder)?;
            }
            Ok(())
        }
    }

    impl<K, H> Decode for HashSet<K, H>
    where
        K: Decode + Eq + Hash,
        H: BuildHasher + Default
    {
        #[cfg_attr(feature = "inline-more", inline)]
        fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
            let len = usize::decode(decoder)?;
            decoder.claim_container_read::<K>(len)?;

            let mut result = HashSet::with_capacity_and_hasher(len, H::default());
            for _ in 0..len {
                decoder.unclaim_bytes_read(core::mem::size_of::<K>());
                let k = K::decode(decoder)?;
                result.insert(k);
            }
            Ok(result)
        }
    }
}
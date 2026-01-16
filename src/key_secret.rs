use std::mem::MaybeUninit;

/// A secret key used for signing transactions in Tashi Vertex.
#[repr(C)]
pub struct KeySecret {
    material: [u8; 32],
}

impl KeySecret {
    /// Generates a new secret key.
    ///
    /// This function populates the provided TVKeySecret structure with a newly
    /// generated secret key suitable for signing transactions.
    ///
    pub fn generate() -> Self {
        let mut key = MaybeUninit::<Self>::uninit();

        unsafe {
            tv_key_secret_generate(key.as_mut_ptr());

            // SAFE: tv_key_secret_generate cannot return without initializing the structure
            key.assume_init()
        }
    }
}

#[link(name = "tashi_vertex")]
unsafe extern "C" {
    fn tv_key_secret_generate(secret: *mut KeySecret);
}

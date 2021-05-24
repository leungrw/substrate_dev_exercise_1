#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;




#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::{Randomness}};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;     
		type Randomness: Randomness<Self::Hash>;
	}


    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(Debug))]
    pub struct Kitty {
        pub dna: [u8; 16],
    }

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
    #[pallet::getter(fn next_kitty)]
    pub type NextKitty<T: Config> = StorageValue<_, u32,ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_kitties)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Kitties<T: Config> = StorageDoubleMap<_,Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, Option<Kitty>>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [kitty_id, who]
		KittyCreated(u32, T::AccountId),
	}
	#[pallet::error]
    pub enum Error<T> {
        NoneValue,
        CreateKittyFailed,
    }

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;
			
			let id = Self::next_kitty();

			match id.checked_add(1) {
				None => Err(Error::<T>::CreateKittyFailed)?,
				Some(id) =>  {
					let rand = T::Randomness::random_seed();
					let dna = (id, &who).using_encoded(blake2_128);
					let kitty = Kitty{dna};
					NextKitty::<T>::put(id);
					Kitties::<T>::insert(&who, id, Some(&kitty));

					Self::deposit_event(Event::KittyCreated(id, who));
				}
			}

			// Update storage.
			// <Kitties<T>>::insert(who, something);

			// Emit an event.
			// Self::deposit_event(Event::KittyCreated(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}
	}


}
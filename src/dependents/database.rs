pub mod data {
use serde::{Serialize, Deserialize};

	pub struct Contact<T>{
		email: T,
		first_name: T,
		last_name: T,
	}
}
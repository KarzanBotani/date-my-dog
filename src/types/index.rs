struct Person {
  dateCreated: u32,
  givenName: String, // Or nickname?
  surname: String,
  phone: Optional<String>, // Either phone and/or email
  email: Optional<String>, // Either phone and/or email
  age: u32,
}

enum PetActivityLevel {
  very_low,
  low,
  medium,
  high,
  very_high,
}

enum PetGroup {
  working,
  herding,
  hound,
  sporting,
  non_sporting,
  toy,
  terrier,
}

enum PetGender {
  female,
  male,
  undecided,
}

struct Pet {
  date_created: u32,
  age: u32,

  // ! -------------
  biography: String,
  breed: String,
  castrated: bool,
  gender: PetGender,
  group: PetGroup,
  in_heat: bool,
  owners: Array<String, 5>,

  height: u32,
  width: u32,
  weight: Optional<u32>,
  profile: Optional<String>,

  profile_picture: String,
}

struct Address {
  street: String,
  coordinates: Optional, // TODO: Map for GPS coordinates
  city: String,          // * Auto-select the city you are located in
  area: String,
}

struct DogPark {
  size: u32,              // * In square meters
  address: Address,       // * Address of the dog park
  currently_active: bool, // * Show which dogs are currently at thte park
  next_planned_visit: Optional<u32>,
}

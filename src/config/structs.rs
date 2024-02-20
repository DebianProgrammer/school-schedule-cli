use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub monday: Monday,
    pub tuesday: Tuesday,
    pub wednesday: Wednesday,
    pub thursday: Thursday,
    pub friday: Friday,
    pub saturday: Saturday,
    pub sunday: Sunday
}

#[derive(Deserialize, Debug)]
pub struct Monday {
    pub times: String
}

#[derive(Deserialize, Debug)]
pub struct Tuesday {
    pub times: String
}

#[derive(Deserialize, Debug)]
pub struct Wednesday {
    pub times: String
}

#[derive(Deserialize, Debug)]
pub struct Thursday {
    pub times: String
}

#[derive(Deserialize, Debug)]
pub struct Friday {
    pub times: String
}

#[derive(Deserialize, Debug)]
pub struct Saturday {
    pub times: String
}

#[derive(Deserialize, Debug)]
pub struct Sunday {
    pub times: String
}



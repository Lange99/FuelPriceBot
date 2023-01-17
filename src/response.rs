
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct fuel{
    pub id: i64,
    pub price: f64,
    pub name: String,
    pub fuelId: i16,
    pub isSelf: bool,
}
#[derive(serde::Serialize, serde::Deserialize, Debug,Clone)]
pub struct location{
    pub lat: f64,
    pub lng: f64,
}
#[derive(serde::Serialize, serde::Deserialize, Debug,Clone)] 
pub struct station{
    pub id: i64,
    pub name: String,
    pub fuels: Vec<fuel>,
    pub location: location,
    pub insertDate: String,
    pub address: String,
    pub brand: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct response_struct{
    pub success: bool,
    pub center: location,
    pub results: Vec<station>
}

impl station{
    pub fn get_price_for_fuel(&self, fuel_id: i16) -> f64{
        let mut prices: Vec<f64> = Vec::new();
        for fuel in &self.fuels{
            if fuel.fuelId == fuel_id{
                prices.push(fuel.price);
            }
        }
        if prices.len() == 0{
            return 0.0;
        } else {
            //sort the vector
            prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
            //return the lowest price
            return prices[0];
        }
    }

    pub fn parse_date(&self) -> String{
        let date = self.insertDate.clone();
        let newdate=date.split(":").collect::<Vec<&str>>()[0].to_string();
        let returndate = newdate.split("T").collect::<Vec<&str>>()[0].to_string();
        returndate
    }
}


    

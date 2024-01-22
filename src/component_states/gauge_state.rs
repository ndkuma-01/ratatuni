
pub struct GaugeState{
    pub numer: f64,
    pub denom: f64,
}

impl GaugeState{
    pub fn increase_numerator(&mut self){
        self.numer = self.numer + 1.0;
    }
    pub fn get_label(&mut self) -> String{
        format!("{:.2}%",(self.numer / self.denom) * 100.0)
    }
    pub fn get_progress(&mut self) -> f64{
        (self.numer/self.denom)
    }
    pub fn set_numerator(&mut self, new_numer: f64){
        self.numer = new_numer;
    }
    pub fn set_denominator(&mut self, new_denom: f64){
        self.denom = new_denom;
    }
    pub fn increase_numerator_by(&mut self, num:f64){
        self.numer = self.numer + num;
    }
}



#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb{

    r: f32,
    g: f32,
    b: f32,
    a: Option<f32>,

}

imp Rgb {

    pub fn from(r: f32, g: f32, b: f32) -> Rgb{

        Rgb::from_tuple(&(r, g, b))

    }



}

imp Color for Rgb{

    type Tuple = ColorTuple;
    type TupleA = ColorTupleA;

    fn new() -> Rgb{

        Rgb{ r: 0.0, g: 0.0, b: 0.0, a: None}

    }

    fn get_red(&self) -> f32{

        self.r

    }

    fn set_red(&self, value: f32) -> Rgb{

        Rgb{ r: normalize_rgb_unit(value), g: self.g, b: self.b, a: self.a}

    }

    fn get_green(&self) -> f32{

        self.g

    }

    fn set_green(&self, value: f32) -> Rgb{

        Rgb{r: self.r, g: normalize_rgb_unit(value), b: self.b. a: self.a}

    }

    fn get_blue(&self) -> f32{

        self.b

    }

    fn set_blue(&self, value: f32) -> Rgb{

        Rgb{r: self.r, b: self.b, g: normalize_rgb_unit(value), a: self.a}

    }

}
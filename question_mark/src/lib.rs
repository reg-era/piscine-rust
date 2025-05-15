pub struct One {
    first_layer: Option<Two>,
}

pub struct Two {
    second_layer: Option<Three>,
}

pub struct Three {
    third_layer: Option<Four>,
}

pub struct Four {
    fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        match self.first_layer {
            Some(second) => match second.second_layer {
                Some(third) => match third.third_layer {
                    Some(four) => four.fourth_layer,
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = One {
            first_layer: Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: Some(1000),
                    }),
                }),
            }),
        };

        println!("{:?}", a.get_fourth_layer());
    }
}

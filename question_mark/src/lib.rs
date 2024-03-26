pub struct One {
    pub first_layer: Option<Two>,
}
pub struct Two {
    pub second_layer: Option<Three>,
}
pub struct Three {
    pub third_layer: Option<Four>
}
pub struct Four {
    pub fourth_layer: Option<u16>
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        self.first_layer.as_ref()?
            .second_layer.as_ref()?
            .third_layer.as_ref()?
            .fourth_layer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn question_mark_test() {
        let a = One {
            first_layer: Some(Two {
                second_layer: Some(Three {
                    third_layer: Some(Four {
                        fourth_layer: Some(1000)
                    })
                })
            })
        };

        assert_eq!(1000, a.get_fourth_layer().expect("Fourth layer is a None"))
    }
}
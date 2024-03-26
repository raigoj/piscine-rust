
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn swap(self, first: u8, second: u8) -> Color {
        let mut arr = self.to_arr();
        let i1 = arr.iter().position(|&v| v == first);
        let i2 = arr.iter().position(|&v| v == second);
        if i1.is_none() || i2.is_none() {
            return self
        }
        let i1 = i1.unwrap();
        let i2 = i2.unwrap();
        arr.swap(i1, i2);
        Color::from_arr(arr)
    }
    fn to_arr(&self) -> [u8; 4] {
        [
            self.r,
            self.g,
            self.b,
            self.a,
        ]
    }
    fn from_arr(arr: [u8; 4]) -> Color {
        Color {
            r: arr[0],
            g: arr[1],
            b: arr[2],
            a: arr[3],
        }
    }
}
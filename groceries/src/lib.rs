pub fn insert(vec: &mut Vec<String>, val: String) {
    return vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> &String {
    return &vec[index];
}

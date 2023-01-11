#[macro_export]
macro_rules! hashmap {
    ($($( $key: expr => $val: expr )+$(,)?)*) => {{
         let mut map = ::std::collections::HashMap::new();
         $($( map.insert($key, $val); )*)*
         map
    }}
}
// macro_rules! hashmap {
//     () => {
//         {
//             let mut hm = ::std::collections::HashMap::new();
//             hm
//         }
//     };
//     ($($k:expr => $v:expr),+ $(,)?) => {
//         {
//             use ::std::collections::HashMap;
//             let mut hm = HashMap::new();
//             $(
//                 hm.insert($k, $v);
//             )*
//             hm
//         }
//     };
// }

fn test() {
    let h = hashmap!('a' => 1 'b' => 2);
}

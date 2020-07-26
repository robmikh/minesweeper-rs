mod guid;

use guid::Guid;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn winrt_guid(input: TokenStream) -> TokenStream {
    let input = {
        let mut input_string = String::new();
        println!("{:#?}", input);
        for part in input {
            match part {
                TokenTree::Literal(part) => input_string.push_str(&part.to_string()),
                TokenTree::Ident(part) => input_string.push_str(&part.to_string()),
                TokenTree::Punct(part) => input_string.push(part.as_char()),
                _ => assert!(false, "Invalid GUID string"),
            }
        }
        input_string
    };
    let guid = Guid::from(input.as_str());
    let data4 = {
        let mut data4 = String::new();
        data4.push_str("[");
        for data in &guid.data4 {
            data4.push_str(&format!("{}, ", data));
        }
        data4.pop();
        data4.pop();
        data4.push_str("]");
        data4
    };
    let output = format!("winrt::Guid::from_values({}, {}, {}, {})", guid.data1, guid.data2, guid.data3, data4);
    output.parse().unwrap()
}
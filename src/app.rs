use regex::Regex;
use std::collections::HashSet;
use web_sys::*;
use yew::prelude::*;
use yew::Callback;

const article: &str = r"once there were two mice. they were friends. one mouse lived in the country; the other mouse lived in the city. after many years the country mouse saw the city mouse; he said, 'do come and see me at my house in the country.' so the city mouse went. the city mouse said, 'this food is not good, and your house is not good. why do you live in a hole in the field? you should come and live in the city. you would live in a nice house made of stone. you would have nice food to eat. you must come and see me at my house in the city.'

　　the country mouse went to the house of the city mouse. it was a very good house. nice food was set ready for them to eat. but just as they began to eat they heard a great noise. the city mouse cried, ' run! run! the cat is coming!' they ran away quickly and hid.

　　after some time they came out. when they came out, the country mouse said, 'i do not like living in the city. i like living in my hole in the field. for it is nicer to be poor and happy, than to be rich and afraid.'";

fn extract_word(text: String) -> Vec<String> {
    // 空格

    let reg: Regex = Regex::new(r"\s|\W|\d|_").unwrap();
    let words: HashSet<&str> = reg.split(&text).collect();
    let mut words: HashSet<String> = words
        .iter()
        .map(|v| {
            //去除单个字
            if v.len() <= 1 {
                "".to_string()
            } else {
                v.to_string().to_lowercase()
            }
        })
        .collect();
    words.remove(&String::from(""));

    let mut vec = words.into_iter().collect::<Vec<String>>();
    vec.sort();
    vec
}
#[function_component(App)]
pub fn app() -> Html {
    let textarea_node_ref: NodeRef = use_node_ref();

    let text: UseStateHandle<String> = use_state(|| article.to_string());
    let oninput: Callback<InputEvent> = {
        let textarea_node_ref: NodeRef = textarea_node_ref.clone();

        let text = text.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(textarea_el) = textarea_node_ref.cast::<HtmlTextAreaElement>() {
                text.set(textarea_el.value());
            }
        })
    };

    // 创建分隔符
    let separator_node_ref: NodeRef = use_node_ref();
    let mut separator: String = "\n".to_string();
    if let Some(separator_el) = separator_node_ref.cast::<HtmlTextAreaElement>() {
        separator = separator_el.value();
    }

    // 合并字串
    let words = extract_word(text.to_string());
    let mut words_str: String = String::new();

    for word in &words {
        words_str.push_str(word.as_str());
        words_str.push_str(&separator.clone());
    }

    html! {
            <div>
              <h2>{"请在下方输入您想要提取单词的文章"}</h2>

              <textarea
                ref={textarea_node_ref}
                {oninput}
                name=""
                id=""
                cols="30"
                rows="10"
                value={text.to_string()}
                placeholder="这里输入你需要拆分的文字"
              ></textarea>
              <br />
              <h2>{"请在下面输入输出分隔符,默认为回车"}</h2>
              <br />
              <textarea value="\n" ref={separator_node_ref}/>
              <h2>{ format!("共提取到{}个单词",words.len())   }</h2>
              <p>{"注意：并不区分单词的正确性"}</p>
              <textarea
                name=""
                id=""
                cols="30"
                rows="10"
                value={words_str}
              ></textarea>
            </div>
    }
}

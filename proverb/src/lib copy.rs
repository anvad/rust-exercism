pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => "".to_string(),
        _ => {
            let mut lines = list
                .windows(2)
                .map(|slice| format!("For want of a {} the {} was lost.", slice[0], slice[1]))
                .collect::<Vec<_>>();
            lines.push(format!(
                "And all for the want of a {}.",
                list.first().unwrap()
            ));
            lines.join("\n")
        }
    }
}

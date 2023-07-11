fn parse_keyword(keyword: &str) -> Result<(), ()> {
    Ok(())
}

fn parse_cmd_line(cmd: &str) -> Result<(), ()> {
    for keyword in cmd.split_whitepace() {
        parse_keyword(keyword);
    }
    Ok(())
}
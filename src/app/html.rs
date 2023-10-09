use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct HtmlContent {
    pub header_tags: Option<HashMap<String, String>>,
    pub title: Option<String>,
    pub body: Option<String>,
}

impl HtmlContent {
    pub fn to_string(self) -> String {

        log::trace!("stringifing HtmlContent: {:?}", self);

        // TODO: move this somewhere else

        let header_start = r#"<!doctype html>
<html lang="en-US">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="static/bulma.css"/>
    <link rel="stylesheet" href="static/fontawesome.css"/>
    <link rel="stylesheet" href="static/solid.css"/>
    <link rel="stylesheet" href="static/custom.css"/>
    <script type="text/javascript" src="static/modal.js"></script>
"#;

        let header_end = "</head>\n";

        let title = match self.title {
            Some(title) => {
                "    <title>".to_string() + &title + "</title>\n"
            },
            None => "".to_string(),
        };

        let header = header_start.to_string() + &title + header_end;


        let body_content =  match self.body {
            Some(body) => body,
            None => "NO PAGE BODY".to_string()
        };
        let body = "<body>\n".to_string() + &body_content + "</body>";

        header + &body + "\n</html>"
    }
}

pub struct HljsService();

impl HljsService {

    pub fn new() -> Self {
        HljsService()
    }

    pub fn highlight(&mut self) {
        js! {
            document.querySelectorAll("pre code")
                .forEach(block => hljs.highlightBlock(block));
        };
    }
}

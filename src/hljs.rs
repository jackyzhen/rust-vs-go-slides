// wrapper around the hljs js lib

pub struct HljsService();

impl HljsService {
    pub fn new() -> Self {
        HljsService()
    }

    pub fn highlight(&mut self) {
        js! {
            document.querySelectorAll("pre code")
                .forEach((block, i )=> {
                    i === 0 && hljs.highlightBlock(block)
                });
        }
    }
}

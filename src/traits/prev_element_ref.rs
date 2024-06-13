use scraper::{ElementRef, Node};

pub trait PrevElementRef {
    fn get_prev_element_ref<'a>(&'a self, element: ElementRef<'a>) -> Option<ElementRef<'a>> {
        let prev_siblings = element.prev_siblings();

        for prev_sibling in prev_siblings {
            if let Node::Element(_) = prev_sibling.value() {
                return ElementRef::wrap(prev_sibling);
            }
        }

        None
    }
}

use crate::{result::Result};

#[derive(Debug)]
pub enum Element<'a> {
    Single(ElementItem<'a>),
    Repeating(Vec<ElementItem<'a>>),
}

impl<'a> Element<'a> {
    pub fn from_bytes(bytes: &'a [u8], config: &ElementConfig) -> Result<Self> {
        if bytes.contains(&config.repetition_separator) {
            let mut occurrences = Vec::new();
            for part in bytes.split(|&b| b == config.repetition_separator) {
                let item = Self::parse_item(part, config);
                occurrences.push(item);
            }
            return Ok(Element::Repeating(occurrences));
        }

        Ok(Element::Single(Self::parse_item(bytes, config)))
    }

    fn parse_item(bytes: &'a [u8], config: &ElementConfig) -> ElementItem<'a> {
        if !bytes.contains(&config.subelement_separator) {
            return ElementItem::Simple(bytes);
        }

        let components = bytes
            .split(|&b| b == config.subelement_separator)
            .collect::<Vec<&[u8]>>();

        ElementItem::Composite(components)
    }
}

#[derive(Debug)]
pub enum ElementItem<'a> {
    Simple(&'a [u8]),
    Composite(Vec<&'a [u8]>),
}

#[derive(Debug, Clone)]
pub struct ElementConfig {
    pub subelement_separator : u8,
    pub repetition_separator : u8,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_element_ok() {
        let payload = b"HELLO";
        let config =  ElementConfig { subelement_separator: b':', repetition_separator: b'^' };

        let elem = Element::from_bytes(payload, &config).unwrap();

        match elem {
            Element::Single(ElementItem::Simple(body)) => {
                assert_eq!(body, b"HELLO");
                assert_eq!(body.len(), 5);
            }
            _ => panic!("expected single simple element"),
        }
    }

    #[test]
    fn composite_element_ok() {
        let payload = b"AA:BB:CC";
        let config = ElementConfig { subelement_separator: b':', repetition_separator: b'^' };

        let elem = Element::from_bytes(payload, &config).unwrap();

        match elem {
            Element::Single(ElementItem::Composite(components)) => {
                assert_eq!(components, vec![b"AA".as_ref(), b"BB".as_ref(), b"CC".as_ref()]);
            }
            _ => panic!("expected single composite element"),
        }
    }

    #[test]
    fn repeating_simple_elements_ok() {
        let payload = b"HELLO^WORLD";
        let config = ElementConfig { subelement_separator: b':', repetition_separator: b'^' };

        let elem = Element::from_bytes(payload, &config).unwrap();

        match elem {
            Element::Repeating(occurrences) => {
                assert_eq!(occurrences.len(), 2);
                match &occurrences[0] {
                    ElementItem::Simple(body) => assert_eq!(*body, b"HELLO"),
                    _ => panic!("expected simple occurrence"),
                }
                match &occurrences[1] {
                    ElementItem::Simple(body) => assert_eq!(*body, b"WORLD"),
                    _ => panic!("expected simple occurrence"),
                }
            }
            _ => panic!("expected repeating element"),
        }
    }

    #[test]
    fn repeating_composite_elements_ok() {
        let payload = b"A:1^B:2";
        let config = ElementConfig { subelement_separator: b':', repetition_separator: b'^' };

        let elem = Element::from_bytes(payload, &config).unwrap();

        match elem {
            Element::Repeating(occurrences) => {
                assert_eq!(occurrences.len(), 2);

                match &occurrences[0] {
                    ElementItem::Composite(components) => {
                        assert_eq!(*components, vec![b"A".as_ref(), b"1".as_ref()]);
                    }
                    _ => panic!("expected composite occurrence"),
                }

                match &occurrences[1] {
                    ElementItem::Composite(components) => {
                        assert_eq!(*components, vec![b"B".as_ref(), b"2".as_ref()]);
                    }
                    _ => panic!("expected composite occurrence"),
                }
            }
            _ => panic!("expected repeating element"),
        }
    }

    #[test]
    fn empty_element_fails() {
        let payload = b"";
        let config = ElementConfig { subelement_separator: b':', repetition_separator: b'^' };

        let res = Element::from_bytes(payload, &config).unwrap();

        match res {
            Element::Single(ElementItem::Simple(body)) => {
                assert_eq!(body, b"");
                assert_eq!(body.len(), 0);
            }
            _ => panic!("expected single simple empty element"),
        }
    }

    #[test]
    fn empty_subelement_fails() {
        let payload = b"AA::BB";
        let config = ElementConfig { subelement_separator: b':', repetition_separator: b'^' };

        let res = Element::from_bytes(payload, &config).unwrap();

        match res {
            Element::Single(ElementItem::Composite(components)) => {
                assert_eq!(components, vec![b"AA".as_ref(), b"".as_ref(), b"BB".as_ref()]);
            }
            _ => panic!("expected single composite element"),
        }
    }
}
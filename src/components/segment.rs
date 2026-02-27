use crate::{components::{element::{Element, ElementConfig}}, error::Error, result::Result};

#[derive(Debug, Clone)]
pub struct SegmentConfig {
    pub terminator: u8,
    pub element_separator: u8,
}

#[derive(Debug, Clone)]
pub struct Segment {
    pub segment_id : Vec<u8>,
    pub body : Vec<u8>,
}

impl Segment {
    pub fn from_bytes(bytes: &[u8], config: &SegmentConfig) -> Result<Self> {
        let has_segment_terminator = bytes.last() == Some(&config.terminator);
        
        let bytes_to_parse = if has_segment_terminator {
            &bytes[..bytes.len() - 1]
        } else {
            bytes
        };

        let mut curr_pos = 0;

        while curr_pos < bytes_to_parse.len()
            && bytes_to_parse[curr_pos] != config.element_separator
        {
            curr_pos += 1;
        }

        if curr_pos == bytes_to_parse.len() {
            return Err(Error::InvalidSegment("missing element separator"));
        }

        let (segment_id_bytes, rest_with_sep) = bytes_to_parse.split_at(curr_pos);

        if segment_id_bytes.is_empty() || segment_id_bytes.len() > 4 {
            return Err(Error::InvalidSegment("invalid segment id length"));
        }
        
        let body_bytes = if rest_with_sep.len() > 1 {
            &rest_with_sep[1..]
        } else {
            &bytes_to_parse[bytes_to_parse.len()..bytes_to_parse.len()]
        };

        Ok(Segment {
            segment_id: segment_id_bytes.to_vec(),
            body: body_bytes.to_vec(),
        })
    }

    pub fn elements<'a>(
        &'a self,
        seg_config: &'a SegmentConfig,
        elem_config: &'a ElementConfig,
    ) -> impl Iterator<Item = Result<Element<'a>>> + 'a {
        self.body
            .split(move |&b| b == seg_config.element_separator)
            .map(move |part| Element::from_bytes(part, elem_config))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn segment_id_should_return_ok() {
        let payload = b"ISA*REST";
        let seg_config = SegmentConfig { terminator: b'~', element_separator: b'*' };

        let res = Segment::from_bytes(payload, &seg_config).unwrap();

        assert_eq!(res.segment_id, b"ISA");
        assert_eq!(res.body, b"REST");
    }

    #[test]
    fn segment_with_terminator_includes_terminator_in_len() {
        let payload = b"ISA*REST~";
        let seg_config = SegmentConfig { terminator: b'~', element_separator: b'*' };

        let res = Segment::from_bytes(payload, &seg_config).unwrap();

        assert_eq!(res.segment_id, b"ISA");
        assert_eq!(res.body, b"REST");
    }

    #[test]
    fn body_can_be_empty() {
        let payload = b"ISA*";
        let seg_config = SegmentConfig { terminator: b'~', element_separator: b'*' };

        let res = Segment::from_bytes(payload, &seg_config).unwrap();

        assert_eq!(res.segment_id, b"ISA");
        assert_eq!(res.body, b"");
    }

    #[test]
    fn header_empty_should_fail() {
        let payload = b"*REST";
        let seg_config = SegmentConfig { terminator: b'~', element_separator: b'*' };

        let res = Segment::from_bytes(payload, &seg_config);

        assert!(res.is_err());
    }

    #[test]
    fn missing_separator_should_fail() {
        let payload = b"ISARESTOFEDI";
        let seg_config = SegmentConfig { terminator: b'~', element_separator: b'*' };

        let res = Segment::from_bytes(payload, &seg_config);

        assert!(res.is_err());
    }
}